use std::{time::{Instant, Duration}, collections::HashMap};

use redis::{Client, Commands};
use anyhow::{Result, Ok};
use crate::models::{BasicPackageData, PackageData};

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
}

#[cfg(test)]
mod test {
    use super::RedisDb;
    use anyhow::{Result, Ok};
    use super::DbActions;

    #[tokio::test]
    async fn ss() -> Result<()> {
        let mut db = RedisDb::try_new()?;
        let result = db.get_most_voted_pkgs(5).await?;
        print!("{:?}", result.result);
        Ok(())
    }
}