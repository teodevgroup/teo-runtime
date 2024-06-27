use std::sync::Arc;
use serde::Serialize;
use super::super::index::Type;
use crate::sort::Sort;

#[derive(Debug, Clone, PartialEq)]
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

impl Serialize for Index {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}