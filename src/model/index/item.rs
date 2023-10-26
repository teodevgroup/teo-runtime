use serde::Serialize;
use crate::sort::Sort;

#[derive(Debug, PartialEq, Serialize, Hash, Clone)]
pub struct Item {
    pub field: String,
    pub sort: Sort,
    pub len: Option<usize>,
}

impl Item {

    pub fn new(field: String, sort: Sort, len: Option<usize>) -> Self {
        Self { field, sort, len }
    }
}