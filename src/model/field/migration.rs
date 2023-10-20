use serde::Serialize;
use teo_teon::Value;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub struct Migration {
    pub renamed: Vec<String>,
    pub version: Option<String>,
    pub default: Option<Value>,
    pub run: Option<Pipeline>,
    pub priority: Option<i64>,
}