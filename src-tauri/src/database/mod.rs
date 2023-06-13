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

use crate::models::{BasicPackageData, PackageData};

#[derive(Serialize, Deserialize, Debug)]
pub struct DbResponse<T: Serialize> {
    pub result: T,
    pub duration: Duration
}

#[async_trait]
pub trait DbActions {
    async fn get_custom_query_time(&mut self, query: &str) -> Result<Duration>;
    async fn run_custom_query(&mut self, query: &str) -> Result<DbResponse<String>>;
    async fn sort_pkgs_by_field_with_limit(&mut self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>>;
    async fn get_most_voted_pkgs(&mut self, number: u32) -> Result<DbResponse<Vec<BasicPackageData>>>;
    async fn insert_pkg(&mut self, pkg: &PackageData) -> Result<DbResponse<()>>;
    async fn get_pkg(&mut self, pkg_name: &str) -> Result<DbResponse<PackageData>>;
}
