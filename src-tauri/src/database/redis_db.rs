use std::time::{Instant, Duration};

use redis::Client;
use anyhow::{Result, Ok};
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
    async fn get_custom_query_time(&self, query: &str) -> Result<Duration> {
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

    async fn run_custom_query(&self, query: &str) -> Result<DbResponse<String>> {
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

    async fn sort_pkgs_by_field_with_limit(&self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        let mut connection = self.client.get_connection()?;
        let mut cmd = redis::cmd("SORT");
        cmd.arg(&["pkgs_set", "by", &format!("pkgs:*->{}", field), "limit", &limit_start.to_string(), &limit_end.to_string(), "DESC"]);

        let start = Instant::now();
        let result: Vec<String> = cmd.query(&mut connection)?;
        let duration = start.elapsed();

        Ok(DbResponse { result, duration })
    }
}