use std::time::{Instant, Duration};

use skytable::{Query, Connection, actions::Actions, ddl::Ddl, types::{IntoSkyhashBytes, FromSkyhashBytes}, SkyResult};
use anyhow::{Result, Ok, bail};
use crate::models::{Comment, AdditionalPackageData, PackageDependency, BasicPackageData};
use async_trait::async_trait;
use std::cmp::Ordering::Equal;

use super::{DbActions, DbResponse};

pub struct SkytableClient {
    connection: Connection
}

impl SkytableClient {
    pub fn try_new() -> Result<Self> {
        Ok(SkytableClient { 
            connection: Connection::new("127.0.0.1", 2003)?
        })
    }

    fn get_all_basic_package_data(&mut self) -> Result<Vec<BasicPackageData>> {
        self.connection.switch("pkgs:basic")?;
        let count = self.connection.dbsize()?;
        let keys: Vec<String> = self.connection.lskeys(count)?;
        let mut result: Vec<BasicPackageData> = Vec::with_capacity(count as usize);

        for key in &keys {
            let package: BasicPackageData = self.connection.get(key)?;
            result.push(package);
        }
        Ok(result)
    }
}

//TODO use list<binstr> ...?
#[async_trait]
impl DbActions for SkytableClient {
    async fn get_custom_query_time(&mut self, query: &str) -> Result<Duration> {
        let parts: Vec<&str> = query.split(" ").collect();

        let start = Instant::now();
        self.connection.run_query_raw(Query::from(parts))?;
        let duration = start.elapsed();

        Ok(duration)
    }
    
    async fn run_custom_query(&mut self, query: &str) -> Result<DbResponse<String>> {
        let parts: Vec<&str> = query.split(" ").collect();

        let start = Instant::now();
        let respone = self.connection.run_query_raw(Query::from(parts))?;
        let duration = start.elapsed();

        let result = respone.try_element_into()?;
        Ok(DbResponse { result, duration })
    }
    
    async fn sort_pkgs_by_field_with_limit(&mut self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        let start = Instant::now();        
        let mut packages = self.get_all_basic_package_data()?;

        sort_values_by(&mut packages, field)?;
        
        let result: Vec<String> = packages.iter().rev()
            .skip(limit_start as usize)
            .take((limit_end - limit_start) as usize)
            .map(|v| v.name.clone())
            .collect();
        let duration = start.elapsed();
        Ok(DbResponse { result, duration })
    }
    async fn get_most_voted_pkgs(&mut self, number: u32) -> Result<DbResponse<Vec<BasicPackageData>>> {
        let start = Instant::now();        
        let mut packages = self.get_all_basic_package_data()?;

        sort_values_by(&mut packages, "votes")?;
        let result: Vec<BasicPackageData> = packages.into_iter().rev()
            .take(number as usize)
            .collect();
        let duration = start.elapsed();
        Ok(DbResponse { result, duration })
    }
}

fn sort_values_by(data: &mut Vec<BasicPackageData>, key: &str) -> Result<()> {
    match key {
       "name" => data.sort_by_key(|k| k.name.clone()),
       "version" => data.sort_by_key(|k| k.version.clone()),
       "path_to_additional_data" => data.sort_by_key(|k| k.path_to_additional_data.clone()),
       "votes" => data.sort_by_key(|k| k.votes),
       "popularity" => data.sort_by(|a, b| a.popularity.partial_cmp(&b.popularity).unwrap_or(Equal)),
       "description" => data.sort_by_key(|k| k.description.clone()),
       "maintainer" => data.sort_by_key(|k| k.maintainer.clone()),
       "last_updated" => data.sort_by_key(|k| k.last_updated.clone()),
       _ => bail!("Unsuported field")
    }
    Ok(())
}

//TODO ----------from scrapper -----------

impl IntoSkyhashBytes for &BasicPackageData {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Cannot serialize PackageData to Vec<u8>")
    }
}

impl FromSkyhashBytes for BasicPackageData {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let bytes: Vec<u8> = element.try_element_into()?;
        serde_json::from_slice(&bytes)
            .map_err(|e| skytable::error::Error::ParseError(e.to_string()))
    }
}

impl IntoSkyhashBytes for &AdditionalPackageData {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Cannot serialize AdditionalPackageData to Vec<u8>")
    }
}

impl FromSkyhashBytes for AdditionalPackageData {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let bytes: Vec<u8> = element.try_element_into()?;
        serde_json::from_slice(&bytes)
            .map_err(|e| skytable::error::Error::ParseError(e.to_string()))
    }
}

impl IntoSkyhashBytes for &Comment {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Cannot serialize Comment to Vec<u8>")
    }
}

impl FromSkyhashBytes for Comment {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let bytes: Vec<u8> = element.try_element_into()?;
        serde_json::from_slice(&bytes)
            .map_err(|e| skytable::error::Error::ParseError(e.to_string()))
    }
}

impl IntoSkyhashBytes for &PackageDependency {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Cannot serialize PackageDependency to Vec<u8>")
    }
}

impl FromSkyhashBytes for PackageDependency {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let bytes: Vec<u8> = element.try_element_into()?;
        serde_json::from_slice(&bytes)
            .map_err(|e| skytable::error::Error::ParseError(e.to_string()))
    }
}
