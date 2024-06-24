use std::sync::Arc;
use serde::Serialize;
use super::index::r#type::Type;
use crate::sort::Sort;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Index {
    inner: Arc<Inner>
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    r#type: Type,
    name: String,
    sort: Sort,
    length: Option<usize>,
}

impl Index {

    pub fn new(r#type: Type, name: String, sort: Sort, length: Option<usize>) -> Self {
        Self {
            inner: Arc::new(Inner {
                r#type,
                name,
                sort,
                length,
            })
        }
    }

    pub fn r#type(&self) -> Type {
        self.inner.r#type
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_str()
    }

    pub fn sort(&self) -> Sort {
        self.inner.sort
    }

    pub fn length(&self) -> Option<usize> {
        self.inner.length
    }
}