use std::time::Instant;


use super::{DbActions, DbResponse};
use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root};

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

#[derive(Debug, Deserialize)]
pub struct D {
    pub name: String,
    pub votes: i32
}

#[async_trait]
impl DbActions for SurrealDbClient {
    //TODO
    async fn run_custom_query(&self, query: &str) -> Result<DbResponse<String>> {
        let start = Instant::now();
        let mut response = self.db.query(query).await?;
        println!("{:?}", response);
        let result: Option<String> = response.take(0)?;
        let duration = start.elapsed();
        if result.is_some() {
            Ok(DbResponse { result: result.unwrap(), duration })
        } else {
            bail!("Result is empty")
        }
    }

    async fn sort_pkgs_by_field_with_limit(&self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
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
}

#[cfg(test)]
mod test {
    use super::SurrealDbClient;
    use anyhow::{Result, Ok};
    use super::DbActions;

    #[tokio::test]
    async fn ss() -> Result<()> {
        let db = SurrealDbClient::try_new().await?;
        db.sort_pkgs_by_field_with_limit("popularity", 1, 4).await?;
        Ok(())
    }
}