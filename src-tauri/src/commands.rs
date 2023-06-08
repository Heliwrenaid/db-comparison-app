use core::fmt;
use std::error::Error;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::database::{DbResponse, RedisDb, DbActions, SkytableClient};

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

#[tauri::command]
pub fn run_query(query_command: QueryCommand) -> Result<DbResponse<String>, FrontendError> {
    let db_client = get_database_client(query_command.target_db)?;
    let response = db_client.run_custom_query(&query_command.query)?;
    Ok(response)
}


#[tauri::command]
pub fn sort_pkgs_by_field_with_limit(target_db: Db, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>, FrontendError> {
    let db_client = get_database_client(target_db)?;
    let response = db_client.sort_pkgs_by_field_with_limit(field, limit_start, limit_end)?;
    Ok(response)
}

fn get_database_client(target_db: Db) -> Result<Box<dyn DbActions>> {
    match target_db {
        Db::Redis => Ok(Box::new(RedisDb::try_new()?)),
        Db::Skytable => Ok(Box::new(SkytableClient::new())),
        Db::SurrealDb => todo!()
    }
}