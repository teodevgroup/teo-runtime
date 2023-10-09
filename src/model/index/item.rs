use serde::Serialize;
use crate::sort::Sort;

#[derive(Debug, PartialEq, Serialize)]
pub struct Item {
    pub field: String,
    pub sort: Sort,
    pub len: Option<usize>,
}