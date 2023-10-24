use serde::Serialize;
use crate::index::Type;
use crate::sort::Sort;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Index {
    pub r#type: Type,
    pub name: String,
    pub sort: Sort,
    pub length: Option<usize>,
}