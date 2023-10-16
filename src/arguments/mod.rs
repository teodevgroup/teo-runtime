use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use serde::{Serialize, Serializer};
use crate::error::Error;
use crate::object::Object;
use crate::result::Result;

#[derive(Clone)]
pub struct Arguments {
    inner: Arc<ArgumentsInner>
}

impl Serialize for Arguments {

    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_map(self.inner.map.iter())
    }
}

impl Debug for Arguments {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Arguments");
        for (k, v) in &self.inner.map {
            debug_struct.field(k.as_str(), &v);
        }
        debug_struct.finish()
    }
}

impl Arguments {

    fn new(map: BTreeMap<String, Object>) -> Self {
        Self {
            inner: Arc::new(ArgumentsInner { map })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.map.is_empty()
    }

    pub fn has(&self, key: impl AsRef<str>) -> bool {
        self.inner.map.contains_key(key.as_ref())
    }

    pub fn get<'a, T: 'a, E>(&'a self, key: impl AsRef<str>) -> Result<T> where E: std::error::Error, T: TryFrom<&'a Object, Error = E> {
        let object = self.get_object_ref(key)?;
        match object.try_into() {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(format!("{e}")))
        }
    }

    pub fn get_object_ref(&self, key: impl AsRef<str>) -> Result<&Object> {
        if let Some(object) = self.inner.map.get(key.as_ref()) {
            Ok(object)
        } else {
            Err(Error::new(format!("argument '{}' is not present", key.as_ref())))
        }
    }

    pub fn get_object(&self, key: impl AsRef<str>) -> Result<Object> {
        self.get_object_ref(key).map(|o| o.clone())
    }
}

struct ArgumentsInner {
    map: BTreeMap<String, Object>
}