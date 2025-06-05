use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub query: String,
    pub operation_name: Option<String>,
    pub variables: Option<HashMap<String, String>>,
    pub extensions: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Serialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Default)]
pub struct Response {
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,
}
