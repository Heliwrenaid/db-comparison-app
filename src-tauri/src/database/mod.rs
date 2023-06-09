use std::time::Duration;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use anyhow::Result;

mod redis_db;
pub use redis_db::RedisDb;

mod skytable_db;
pub use skytable_db::SkytableClient;

mod surreal_db;
pub use surreal_db::SurrealDbClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbResponse<T: Serialize> {
    pub result: T,
    pub duration: Duration
}

#[async_trait]
pub trait DbActions {
    async fn run_custom_query(&self, query: &str) -> Result<DbResponse<String>>;
    async fn sort_pkgs_by_field_with_limit(&self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>>;
}
