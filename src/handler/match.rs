use std::sync::Arc;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct HandlerMatch {
    inner: Arc<Inner>,
}

#[derive(Debug, Clone)]
struct Inner {
    path: Vec<String>,
    name: String,
    captures: IndexMap<String, String>,
    path_without_last: Vec<String>,
}

impl HandlerMatch {

    pub fn new(path: Vec<String>, name: String, captures: IndexMap<String, String>) -> Self {
        let mut path_without_last = path.clone();
        path_without_last.pop();
        Self {
            inner: Arc::new(Inner {
                path,
                name,
                captures,
                path_without_last,
            })
        }
    }

    pub fn path_without_last(&self) -> &Vec<String> {
        &self.inner.path_without_last
    }

    pub fn group_name(&self) -> &str {
        self.path().last().unwrap()
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_str()
    }

    pub fn handler_name(&self) -> &str {
        self.inner.name.as_str()
    }

    pub fn captures(&self) -> &IndexMap<String, String> {
        &self.inner.captures
    }
}