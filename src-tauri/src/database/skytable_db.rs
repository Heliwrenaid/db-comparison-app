use std::time::{Instant, Duration};

use serde::{Deserialize, Serialize};
use skytable::{Query, Connection, actions::Actions, ddl::Ddl, types::{IntoSkyhashBytes, FromSkyhashBytes}, SkyResult};
use anyhow::{Result, Ok, bail};
use crate::models::{Comment, AdditionalPackageData, PackageDependency, BasicPackageData, PackageData};
use async_trait::async_trait;
use std::cmp::Ordering::Equal;

use super::{DbActions, DbResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Comments {
    data: Vec<Comment>
}

#[derive(Debug, Serialize, Deserialize)]
struct Dependencies {
    data: Vec<PackageDependency>
}

pub struct SkytableClient {
    connection: Connection
}

const BASIC_PKGS_TABLE: &str = "pkgs:basic";
const ADDITIONAL_PKGS_TABLE: &str = "pkgs:additional";
const COMMENTS_TABLE: &str = "pkgs:comments";
const DEPENDENCIES_TABLE: &str = "pkgs:dependencies";

impl SkytableClient {
    pub fn try_new() -> Result<Self> {
        Ok(SkytableClient { 
            connection: Connection::new("127.0.0.1", 2003)?
        })
    }

    fn get_all_basic_package_data(&mut self) -> Result<Vec<BasicPackageData>> {
        self.connection.switch(BASIC_PKGS_TABLE)?;
        let count = self.connection.dbsize()?;
        let keys: Vec<String> = self.connection.lskeys(count)?;
        let response: DbResponse<Vec<BasicPackageData>> = self.connection.mget(keys)?;
        Ok(response.result)
    }
}

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

    async fn insert_pkg(&mut self, pkg: &PackageData) -> Result<DbResponse<()>> {
        let start = Instant::now();
        let pkg_name = pkg.basic.name.clone();
        self.connection.switch(BASIC_PKGS_TABLE)?;
        self.connection.set(&pkg_name, &pkg.basic)?;

        self.connection.switch(ADDITIONAL_PKGS_TABLE)?;
        self.connection.set(&pkg_name, &pkg.additional)?;

        self.connection.switch(COMMENTS_TABLE)?;
        self.connection.run_query_raw(Query::new().arg("LSET").arg(&pkg.basic.name))?;
        self.connection.run_query_raw(Query::new().arg("LMOD").arg(&pkg.basic.name).arg("CLEAR"))?;

        for comment in &pkg.comments {
            let query = Query::new().arg("LMOD").arg(&pkg.basic.name).arg("PUSH").arg(comment);
            self.connection.run_query_raw(query)?;
        }

        self.connection.switch(DEPENDENCIES_TABLE)?;
        self.connection.run_query_raw(Query::new().arg("LSET").arg(&pkg.basic.name))?;
        self.connection.run_query_raw(Query::new().arg("LMOD").arg(&pkg.basic.name).arg("CLEAR"))?;

        for dependency in &pkg.dependencies {
            let query = Query::new().arg("LMOD").arg(&pkg.basic.name).arg("PUSH").arg(dependency);
            self.connection.run_query_raw(query)?;
        }

        let duration = start.elapsed();
        Ok(DbResponse { result: (), duration })
    }

    async fn get_pkg(&mut self, name: &str) -> Result<DbResponse<PackageData>> {
        let start = Instant::now();
        self.connection.switch(BASIC_PKGS_TABLE)?;
        let basic: BasicPackageData = self.connection.get(name)?;

        self.connection.switch(ADDITIONAL_PKGS_TABLE)?;
        let additional: AdditionalPackageData = self.connection.get(name)?;

        self.connection.switch(COMMENTS_TABLE)?;
        let comments: Comments = self.connection.run_query(Query::new().arg("LGET").arg(name))?;

        self.connection.switch(DEPENDENCIES_TABLE)?;
        let dependencies: Dependencies = self.connection.run_query(Query::new().arg("LGET").arg(name))?;

        let duration = start.elapsed();
        let result = PackageData {
            basic,
            additional,
            comments: comments.data,
            dependencies: dependencies.data,
        };
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

impl FromSkyhashBytes for DbResponse<Vec<BasicPackageData>> {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let vec_of_bytes: Vec<Vec<u8>> = element.try_element_into()?;
        let mut pkgs: Vec<BasicPackageData> = Vec::new();
        for bytes in vec_of_bytes {
            let pkg: BasicPackageData = serde_json::from_slice(&bytes)
            .map_err(|e| skytable::error::Error::ParseError(e.to_string()))?;
            pkgs.push(pkg);
        }
        skytable::SkyResult::Ok(DbResponse { result: pkgs, duration: Duration::ZERO })
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

impl FromSkyhashBytes for Comments {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let mut comments: Vec<Comment> = Vec::new();
        let comments_bytes: Vec<Vec<u8>> = element.try_element_into()?;
        for comment_bytes in &comments_bytes {
            let comment: Comment = serde_json::from_slice(comment_bytes)
                .map_err(|e| skytable::error::Error::ParseError(e.to_string()))?;
            comments.push(comment);
        }
        skytable::SkyResult::Ok(Comments { data: comments })
    }
}

impl FromSkyhashBytes for Dependencies {
    fn from_element(element: skytable::Element) -> SkyResult<Self> {
        let dependencies_bytes: Vec<Vec<u8>> = element.try_element_into()?;
        let mut dependencies: Vec<PackageDependency> = Vec::new();
        for dependency_bytes in dependencies_bytes {
            let dependency: PackageDependency = serde_json::from_slice(&dependency_bytes)
                .map_err(|e| skytable::error::Error::ParseError(e.to_string()))?;
            dependencies.push(dependency);
        }
        skytable::SkyResult::Ok(Dependencies { data: dependencies })
    }
}

#[cfg(test)]
mod test {
    use super::SkytableClient;
    use anyhow::{Result, Ok};
    use super::DbActions;

    #[tokio::test]
    async fn test_query() -> Result<()> {
        let mut db = SkytableClient::try_new()?;
        let result = db.get_most_voted_pkgs(5).await?;
        println!("{:?}", result);
        Ok(())
    }
}