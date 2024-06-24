use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use crate::comment::Comment;
use crate::r#enum::member::Member;
use crate::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Builder {
    inner: Arc<Inner>
}

#[derive(Debug)]
struct Inner {
    name: String,
    comment: Option<Comment>,
    value: Value,
    data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {

    pub fn new(name: String, value: Value, comment: Option<Comment>) -> Self {
        Self {
            inner: Arc::new(Inner {
                name,
                value,
                comment,
                data: Arc::new(Mutex::new(BTreeMap::new())),
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    pub fn value(&self) -> &Value {
        &self.inner.value
    }

    pub fn data(&self) -> BTreeMap<String, Value> {
        self.inner.data.lock().unwrap().clone()
    }

    pub fn set_data(&self, data: BTreeMap<String, Value>) {
        *self.inner.data.lock().unwrap() = data;
    }

    pub fn insert_data_entry(&self, key: String, value: Value) {
        self.inner.data.lock().unwrap().insert(key, value);
    }

    pub fn remove_data_entry(&self, key: &str) {
        self.inner.data.lock().unwrap().remove(key);
    }

    pub fn data_entry(&self, key: &str) -> Option<Value> {
        self.inner.data.lock().unwrap().get(key).cloned()
    }

    pub(crate) fn build(self) -> Member {
        Member {
            name: self.inner.name.clone(),
            value: self.inner.value.clone(),
            comment: self.inner.comment.clone(),
            data: self.inner.data.lock().unwrap().clone(),
        }
    }
}