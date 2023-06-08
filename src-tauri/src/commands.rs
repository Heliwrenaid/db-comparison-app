use std::time::{Instant, Duration};

use redis::{Commands, FromRedisValue};
use serde::{Serialize, Deserialize};
use skytable::Query;

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
    //TODO: calculate times in query_* methods
    let start = Instant::now();
    let result = match query_command.target_db {
        Db::SurrealDb => query_surrealdb(&query_command.query.trim()),
        Db::Redis => query_redis(&query_command.query.trim()),
        Db::Skytable => query_skytable(&query_command.query.trim())
    };
    let duration = start.elapsed();
    println!("{:?}", result);
    QueryResult { result, duration }
}

fn query_surrealdb(query: &str) -> String {
    format!("SurrealDB: {}", query)
}

fn query_redis(query: &str) -> String {
    let parts: Vec<&str> = query.split(" ").collect();
    if parts.get(0).is_none() {
        return "Error: Missed command".to_owned();
    }
    let mut cmd = redis::cmd(parts.get(0).unwrap());
    parts.iter()
        .skip(1)
        .for_each(|arg| _ = cmd.arg(arg));
    match cmd.query(&mut get_redis_connection()) {
        Ok(data) => data,
        Err(error) => error.to_string()
    }
        
}

fn query_skytable(query: &str) -> String {
    let parts: Vec<&str> = query.split(" ").collect();
    match get_skytable_connection().run_query(Query::from(parts)) {
        Ok(data) => data,
        Err(error) => error.to_string()
    }
}


fn get_redis_connection() -> redis::Connection {
    let redis_host_name = "127.0.0.1";
    let port = "6379";
    let redis_password = "redis";
    let username = "default";
    let uri_scheme = "redis";
    let redis_conn_url = format!("{}://{}:{}@{}:{}", uri_scheme, username, redis_password, redis_host_name, port);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn get_skytable_connection() -> skytable::Connection {
    skytable::Connection::new("127.0.0.1", 2003)
        .expect("Failed to connect to Skytable")
}