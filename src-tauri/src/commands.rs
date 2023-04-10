use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub result: String
}

#[tauri::command]
pub fn run_query(query: &str) -> QueryResult {
    QueryResult { result: query.to_owned() }
}