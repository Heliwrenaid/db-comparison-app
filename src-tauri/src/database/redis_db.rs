use std::{time::{Instant, Duration}, collections::HashMap};

use redis::{Client, Commands};
use anyhow::{Result, Ok, anyhow};
use tauri::regex::internal::Inst;
use crate::models::{BasicPackageData, PackageData, Comment, PackageDependency};

use super::{DbActions, DbResponse};
use async_trait::async_trait;

pub struct RedisDb {
    client: Client
}

impl RedisDb {
    pub fn try_new() -> Result<Self> {
            let redis_host_name = "127.0.0.1";
            let port = "6379";
            let redis_password = "redis";
            let username = "default";
            let uri_scheme = "redis";
            let redis_conn_url = format!("{}://{}:{}@{}:{}", uri_scheme, username, redis_password, redis_host_name, port);
            let client = Client::open(redis_conn_url)?;
            Ok(Self { client })
    }
}

#[async_trait]
impl DbActions for RedisDb {
    async fn get_custom_query_time(&mut self, query: &str) -> Result<Duration> {
        let parts: Vec<&str> = query.split(" ").collect();
        let mut cmd = redis::cmd(parts.get(0).unwrap());
        parts.iter()
            .skip(1)
            .for_each(|arg| _ = cmd.arg(arg));

        let mut connection = self.client.get_connection()?;
        let start = Instant::now();
        cmd.query(&mut connection)?;
        let duration = start.elapsed();
        
        Ok(duration)
    }

    async fn run_custom_query(&mut self, query: &str) -> Result<DbResponse<String>> {
        let parts: Vec<&str> = query.split(" ").collect();
        let mut cmd = redis::cmd(parts.get(0).unwrap());
        parts.iter()
            .skip(1)
            .for_each(|arg| _ = cmd.arg(arg));

        let mut connection = self.client.get_connection()?;
        let start = Instant::now();
        let result: String = cmd.query(&mut connection)?;
        let duration = start.elapsed();

        Ok(DbResponse { result, duration })
    }

    async fn sort_pkgs_by_field_with_limit(&mut self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        let mut connection = self.client.get_connection()?;
        let mut cmd = redis::cmd("SORT");
        cmd.arg(&["pkgs_set", "by", &format!("pkgs:*->{}", field), "limit", &limit_start.to_string(), &limit_end.to_string(), "DESC"]);

        let start = Instant::now();
        let result: Vec<String> = cmd.query(&mut connection)?;
        let duration = start.elapsed();

        Ok(DbResponse { result, duration })
    }

    async fn get_most_voted_pkgs(&mut self, number: u32) -> Result<DbResponse<Vec<BasicPackageData>>> {
        let pkgs_name_response = self.sort_pkgs_by_field_with_limit("votes", 0, number).await?;
        
        let mut connection = self.client.get_connection()?;

        let start = Instant::now();

        let mut result = Vec::new();
        for name in &pkgs_name_response.result {
            let mut pkg_dict: HashMap<String, String> = connection.hgetall(format!("pkgs:{}", name))?;
            pkg_dict.insert("name".into(), name.into());
    
            let pkg = PackageData::try_from(pkg_dict)?;
            result.push(pkg.basic);
        }

        let duration = start.elapsed();
        Ok(DbResponse { result, duration: duration + pkgs_name_response.duration })
    }

    async fn insert_pkg(&mut self, pkg: &PackageData) -> Result<DbResponse<()>> {
        let mut connection = self.client.get_connection()?;
        let start = Instant::now();
        connection.hset_multiple(
            format!("pkgs:{}", pkg.basic.name),
            &[
                ("popularity", pkg.basic.popularity.to_string().as_str()),
                ("last_updated", pkg.basic.last_updated.as_str()),
                ("description", pkg.basic.description.as_str()),
                ("maintainer", pkg.basic.maintainer.as_str()),
                ("version", pkg.basic.version.as_str()),
                ("votes", pkg.basic.votes.to_string().as_str()),
                ("path_to_additional_data", pkg.basic.path_to_additional_data.as_str()),
                ("firstsubmitted", pkg.additional.first_submitted.as_str()),
                ("gitcloneurl", pkg.additional.git_clone_url.as_str()),
                ("submitter", pkg.additional.submitter.as_str()),
                (
                    "confilcts",
                    pkg.additional.confilcts.as_ref().map(|s| s.as_str()).unwrap_or(""),
                ),
                (
                    "provides",
                    pkg.additional.provides.as_ref().map(|s| s.as_str()).unwrap_or(""),

                ),
                (
                    "keywords",
                    pkg.additional.keywords.as_ref().map(|s| s.as_str()).unwrap_or(""),
                ),
                (
                    "license",
                    pkg.additional.license.as_ref().map(|s| s.as_str()).unwrap_or(""),
                ),
            ],
        )?;

        connection.sadd("pkgs_set", &pkg.basic.name)?;

        for (idx, comment) in pkg.comments.iter().enumerate() {
            connection.hset_multiple(
                format!("pkgs:{}:cmnts:{}", pkg.basic.name, idx + 1),
                &[("header", &comment.header), ("content", &comment.content)],
            )?;

            connection.sadd(
                format!("pkgs:{}:cmnts", pkg.basic.name),
                format!("pkgs:{}:cmnts:{}", pkg.basic.name, idx + 1),
            )?;
        }

        for dependency in &pkg.dependencies {
            for dep in &dependency.packages {
                connection.rpush(
                    format!("pkgs:{}:deps:{}", pkg.basic.name, dependency.group),
                    dep,
                )?;
            }

            connection.sadd(
                format!("pkgs:{}:deps", pkg.basic.name),
                format!("pkgs:{}:deps:{}", pkg.basic.name, dependency.group),
            )?;
        }
        let duration = start.elapsed();
        Ok(DbResponse { result: (), duration })
    }

    async fn get_pkg(&mut self, name: &str) -> Result<DbResponse<PackageData>> {
        let mut conn = self.client.get_connection()?;
        let start = Instant::now();
        let mut pkg_dict: HashMap<String, String> = conn.hgetall(format!("pkgs:{}", name))?;
        pkg_dict.insert("name".into(), name.into());

        let mut pkg = PackageData::try_from(pkg_dict).map_err(|e| anyhow!(e))?;

        let cmnts_list: Vec<String> = conn.smembers(format!("pkgs:{}:cmnts", pkg.basic.name))?;

        let mut comments = vec![];

        for cmnt in cmnts_list {
            let cmnt_dict: HashMap<String, String> = conn.hgetall(cmnt)?;
            comments.push(Comment::try_from(cmnt_dict)?);
        }

        pkg.comments = comments;

        let group_list: Vec<String> = conn.smembers(format!("pkgs:{}:deps", pkg.basic.name))?;

        let mut dependencies = vec![];

        for group in group_list {
            let packages: Vec<String> = conn.lrange(&group, 0, -1)?;

            dependencies.push(PackageDependency { group, packages });
        }
        let duration = start.elapsed();
        pkg.dependencies = dependencies;

        Ok(DbResponse { result: pkg, duration })
    }

    async fn remove_comments(&mut self, pkg_name: &str) -> Result<DbResponse<()>> {
        let mut connection = self.client.get_connection()?;
        let start = Instant::now();
        connection.del(format!("pkgs:{}:cmnts", pkg_name))?;
        let duration = start.elapsed();
        Ok(DbResponse { result: (), duration })
    }

    async fn get_packages_occurences_in_deps(&mut self, pkg_deps_names: &Vec<String>) -> Result<DbResponse<HashMap<String, u32>>> {
        let mut connection = self.client.get_connection()?;
        let mut data: HashMap<String, u32> = HashMap::new();
        pkg_deps_names.iter().for_each(|name| _ = data.insert(name.to_owned(), 0));

        let start = Instant::now();
        let all_pkg_names: Vec<String> = connection.smembers("pkgs_set")?;
        for pkg_name in &all_pkg_names {
            let group_list: Vec<String> = connection.smembers(format!("pkgs:{}:deps", pkg_name))?;
            for pkg_dep_name in pkg_deps_names {
                if group_list.contains(&format!("pkgs:{}:deps:{}", pkg_name, pkg_dep_name)) {
                    if data.contains_key(pkg_dep_name) {
                        let count = data.get(pkg_dep_name).unwrap() + 1;
                        data.insert(pkg_dep_name.to_owned(), count);
                    }
                }
            }
        }
        let duration = start.elapsed();        
        Ok(DbResponse { result: data, duration })
    }

}

#[cfg(test)]
mod test {
    use super::RedisDb;
    use anyhow::{Result, Ok};
    use super::DbActions;

    #[tokio::test]
    async fn ss() -> Result<()> {
        let mut db = RedisDb::try_new()?;
        let result = db.get_packages_occurences_in_deps(&vec!["rust".to_string(), "go".to_string(), "sudo".to_string()]).await?;
        print!("{:?}", result.result);
        Ok(())
    }
}