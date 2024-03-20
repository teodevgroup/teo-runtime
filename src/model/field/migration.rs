use serde::Serialize;
use crate::value::Value;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub struct Migration {
    pub renamed: Vec<String>,
    pub version: Option<String>,
    pub default: Option<Value>,
    pub priority: Option<i64>,
}