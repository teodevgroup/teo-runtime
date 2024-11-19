use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use teo_result::{Result, Error};
use crate::Value;

#[derive(Default)]
pub struct LocalValues {
    map: BTreeMap<String, Value>,
}

impl LocalValues {

    #[inline]
    pub fn new() -> LocalValues {
        Self {
            map: BTreeMap::default(),
        }
    }

    pub fn insert<T: 'static + Into<Value>>(&mut self, key: impl Into<String>, val: T) {
        self.map.insert(key.into(), val.into());
    }

    pub fn get<'a, T>(&'a self, key: &str) -> Result<Option<T>> where T: 'a + TryFrom<Value>, Error: From<T::Error> {
        match self.map.get(key) {
            None => Ok(None),
            Some(v) => {
                let v = v.clone();
                let v = T::try_from(v)?;
                Ok(Some(v))
            }
        }
    }

    pub fn contains<T: 'static + Send>(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove<T: 'static + Send>(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }
}

impl Debug for LocalValues {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("LocalValues");
        for (k, v) in &self.map {
            debug_struct.field(&k, &v);
        }
        debug_struct.finish()
    }
}
