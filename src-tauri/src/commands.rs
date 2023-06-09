use core::fmt;
use std::error::Error;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::database::{DbResponse, RedisDb, DbActions, SkytableClient, SurrealDbClient};

#[derive(Serialize, Deserialize, Debug)]
pub enum Db {
    SurrealDb,
    Redis,
    Skytable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCommand {
    target_db: Db,
    query: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontendError {
    message: String
}

impl Error for FrontendError {}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An unknown error occured")
    }
}

impl From<anyhow::Error> for FrontendError {
    fn from(error: anyhow::Error) -> Self {
        FrontendError { message: error.to_string() }
    }
}

// TODO: fix trait object with async
#[tauri::command]
pub async fn run_query(query_command: QueryCommand) -> Result<DbResponse<String>, FrontendError> {
    let response = match query_command.target_db {
        Db::Redis => RedisDb::try_new()?.run_custom_query(&query_command.query).await?,
        Db::Skytable => SkytableClient::new().run_custom_query(&query_command.query).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.run_custom_query(&query_command.query).await?
    };
    Ok(response)
}

#[tauri::command]
pub async fn sort_pkgs_by_field_with_limit(target_db: Db, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.sort_pkgs_by_field_with_limit(field, limit_start, limit_end).await?,
        Db::Skytable => SkytableClient::new().sort_pkgs_by_field_with_limit(field, limit_start, limit_end).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.sort_pkgs_by_field_with_limit(field, limit_start, limit_end).await?
    };
    Ok(response)
}
