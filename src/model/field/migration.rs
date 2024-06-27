use serde::Serialize;
use crate::value::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Migration {
    pub renamed: Vec<String>,
    pub version: Option<String>,
    pub default: Option<Value>,
    pub priority: Option<i64>,
}