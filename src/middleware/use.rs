use serde::Serialize;
use crate::arguments::Arguments;

#[derive(Debug, Serialize)]
pub struct Use {
    pub path: Vec<String>,
    pub arguments: Arguments,
}