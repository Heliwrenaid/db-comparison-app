use std::time::{Instant, Duration};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub result: String,
    pub duration: Duration
}

#[derive(Serialize, Deserialize, Debug)]
enum Db {
    SurrealDb,
    Redis,
    Skytable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCommand {
    target_db: Db,
    query: String
}

#[tauri::command]
pub fn run_query(query_command: QueryCommand) -> QueryResult {
    let start = Instant::now();
    let result = match query_command.target_db {
        Db::SurrealDb => query_surrealdb(&query_command.query),
        Db::Redis => query_redis(&query_command.query),
        Db::Skytable => query_skytable(&query_command.query)
    };
    let duration = start.elapsed();
    QueryResult { result, duration }
}

fn query_surrealdb(query: &str) -> String {
    format!("SurrealDB: {}", query)
}

fn query_redis(query: &str) -> String {
    format!("Redis: {}", query)
}

fn query_skytable(query: &str) -> String {
    format!("Skytable: {}", query)
}