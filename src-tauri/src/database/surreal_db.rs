use std::{time::{Instant, Duration}, collections::HashMap};


use crate::models::{PackageData, BasicPackageData, PackageDependency};

use super::{DbActions, DbResponse};
use anyhow::{Result, Ok};
use async_trait::async_trait;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root, Response};

type SurResult<T> = Result<T, surrealdb::Error>;

pub struct SurrealDbClient {
    db: Surreal<Client>,
}

impl SurrealDbClient {
    pub async fn try_new() -> Result<Self> {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        db.use_ns("aur").use_db("packages").await?;

        Ok(Self { db })
    }
}

fn skip_already_exist_error<T>(res: SurResult<T>) -> Result<()> {
    if let Err(e) = res {
        if !e.to_string().contains("already exists") {
            return Err(e.into());
        }
    }

    Ok(())
}

#[async_trait]
impl DbActions for SurrealDbClient {
    async fn get_custom_query_time(&mut self, query: &str) -> Result<Duration> {
        let start = Instant::now();
        self.db.query(query).await?;
        Ok(start.elapsed())
    }

    async fn run_custom_query(&mut self, query: &str) -> Result<DbResponse<String>> {
        let start = Instant::now();
        let mut response: Response = self.db.query(query).await?;
        let duration = start.elapsed();

        let result: Option<PackageData> = response.take(0)?;
        if let Some(data) = result {
            let result = serde_json::to_string(&data)?;
            return Ok(DbResponse { result, duration });
        }
        Ok(DbResponse { result: "No data found".to_owned(), duration })
    }

    async fn sort_pkgs_by_field_with_limit(&mut self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        let query = 
            format!("SELECT VALUE name FROM (SELECT basic.name as name, basic.{} as key 
                FROM pkgs ORDER BY key DESC LIMIT BY {} START AT {})",
                field,
                limit_end.to_string(),
                limit_start.to_string()
            );

        let start = Instant::now();
        let result: Vec<String> = self.db.query(query).await?.take(0)?;
        let duration = start.elapsed();
        Ok(DbResponse { result, duration })
    }

    async fn get_most_voted_pkgs(&mut self, number: u32) -> Result<DbResponse<Vec<BasicPackageData>>> {
        let query = format!("SELECT VALUE basic from 
            (SELECT basic, basic.votes as votes from pkgs ORDER BY votes DESC LIMIT BY {})", number.to_string());
        
        let start = Instant::now();
        let result: Vec<BasicPackageData> = self.db.query(query).await?.take(0)?;
        let duration = start.elapsed();
        Ok(DbResponse { result, duration })
    }

    async fn insert_pkg(&mut self, pkg: &PackageData) -> Result<DbResponse<()>> {
        let start = Instant::now();
        let _res: SurResult<()> = self
            .db
            .create(("pkgs", &pkg.basic.name))
            .content(&pkg)
            .await;
        let duration = start.elapsed();
        Ok(DbResponse { result: (), duration })
    }

    async fn get_pkg(&mut self, name: &str) -> Result<DbResponse<PackageData>> {
        let start = Instant::now();
        let result: PackageData = self.db.select(("pkgs", name)).await?;
        let duration = start.elapsed();
        Ok(DbResponse { result , duration })
    }

    async fn remove_comments(&mut self, pkg_name: &str) -> Result<DbResponse<()>> {
        let query = format!("UPDATE pkgs SET comments = [] WHERE basic.name = '{}'", pkg_name);
        let duration = self.get_custom_query_time(&query).await?;
        Ok(DbResponse { result: (), duration })
    }

    async fn get_packages_occurences_in_deps(&mut self, pkg_names: &Vec<String>) -> Result<DbResponse<HashMap<String, u32>>> {
        let mut result: HashMap<String, u32> = HashMap::new();
        pkg_names.iter().for_each(|name| _ = result.insert(name.to_owned(), 0));
        
        let start = Instant::now();
        let pkg_deps_names: Vec<Vec<String>> = self.db.query("SELECT VALUE dependencies.group FROM pkgs").await?.take(0)?;
        let pkg_deps_names: Vec<String> = pkg_deps_names.into_iter().flatten().collect();
        for pkg_name in pkg_deps_names {
            if result.contains_key(&pkg_name) {
                let count = result.get(&pkg_name).unwrap() + 1;
                result.insert(pkg_name, count);
            }
        }
        let duration = start.elapsed();
        Ok(DbResponse { result , duration })
    }
}

#[cfg(test)]
mod test {
    use super::SurrealDbClient;
    use anyhow::{Result, Ok};
    use super::DbActions;

    #[tokio::test]
    async fn test_query() -> Result<()> {
        let mut db = SurrealDbClient::try_new().await?;
        let result = db.get_packages_occurences_in_deps(&vec!["rust".to_string(), "go".to_string(), "sudo".to_string()]).await?;
        println!("{:?}", result);
        Ok(())
    }
}