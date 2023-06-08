use std::time::{Instant};

use skytable::{Query, Connection, actions::Actions, ddl::Ddl, types::{IntoSkyhashBytes, FromSkyhashBytes}, SkyResult};
use anyhow::{Result, Ok};
use crate::models::{Comment, AdditionalPackageData, PackageDependency, BasicPackageData};

use super::{DbActions, DbResponse};

pub struct SkytableClient {}

impl SkytableClient {
    pub fn new() -> Self {
        SkytableClient {  }
    }
}

//TODO use list<binstr> ...?
impl DbActions for SkytableClient {
    fn run_custom_query(&self, query: &str) -> Result<DbResponse<String>> {
        let mut connection = get_skytable_connection()?;
        let parts: Vec<&str> = query.split(" ").collect();

        let start = Instant::now();
        let respone = connection.run_query_raw(Query::from(parts))?;
        let duration = start.elapsed();

        let result = respone.try_element_into()?;
        Ok(DbResponse { result, duration })
    }
    
    fn sort_pkgs_by_field_with_limit(&self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        let mut connection = get_skytable_connection()?;
        println!("asd");
        let start = Instant::now();
        connection.switch("pkgs:basic")?;
        let count = connection.dbsize()?;
        let keys: Vec<String> = connection.lskeys(count)?;
        let mut packages: Vec<BasicPackageData> = Vec::with_capacity(count as usize);

        for key in &keys {
            let package: BasicPackageData = connection.get(key)?;
            packages.push(package);
        }
        packages.sort_by_key(|k| k.votes); //TODO: get by field???
        
        let result: Vec<String> = packages.iter().rev()
            .skip(limit_start as usize)
            .take((limit_end - limit_start) as usize)
            .map(|v| v.name.clone())
            .collect();
        let duration = start.elapsed();
        Ok(DbResponse { result, duration })
    }
}

fn get_skytable_connection() -> Result<Connection> {
    Ok(Connection::new("127.0.0.1", 2003)?)
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
