use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use serde::Serializer;

#[derive(Clone)]
pub struct HeaderMap {
    inner: Arc<Inner>
}

impl Debug for HeaderMap {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.collect_map(self.inner.as_ref().fields.lock().unwrap().iter())
    }
}

impl HeaderMap {

    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner::new())
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.inner.fields.lock().unwrap().keys().map(|k| k.to_string()).collect()
    }

    pub fn len(&self) -> usize {
        self.inner.fields.lock().unwrap().len()
    }

    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner.fields.lock().unwrap().contains_key(key.as_ref())
    }

    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.fields.lock().unwrap().insert(key.into(), value.into());
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<String> {
        self.inner.fields.lock().unwrap().get(key.as_ref()).map(|s| s.clone())
    }
}

struct Inner {
    pub fields: Mutex<BTreeMap<String, String>>,
}

impl Inner {

    pub fn new() -> Inner {
        Self {
            fields: Mutex::new(btreemap! {})
        }
    }
}
