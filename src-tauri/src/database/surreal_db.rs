use super::{DbActions, DbResponse};
use anyhow::Result;

pub struct SurrealDbClient {}

impl SurrealDbClient {
    pub fn new() -> Self {
        SurrealDbClient {  }
    }
}

impl DbActions for SurrealDbClient {
    fn run_custom_query(&self, query: &str) -> Result<DbResponse<String>> {
        todo!();
    }

     fn sort_pkgs_by_field_with_limit(&self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        todo!()
    }
}