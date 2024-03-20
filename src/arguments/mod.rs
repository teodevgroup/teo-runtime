use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use maplit::btreemap;
use serde::{Serialize, Serializer};
use teo_result::Error;
use teo_result::Result;
use crate::value::Value;

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

impl Display for Arguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        for (i, (k, v)) in self.inner.map.iter().enumerate() {
            f.write_str(k)?;
            f.write_str(": ")?;
            Display::fmt(v, f)?;
            if i != self.inner.map.len() - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")?;
        Ok(())
    }
}

impl Arguments {

    pub(crate) fn new(map: BTreeMap<String, Value>) -> Self {
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

    pub fn get<'a, T: 'a, E>(&'a self, key: impl AsRef<str>) -> Result<T> where E: std::error::Error, T: TryFrom<&'a Value, Error = E> {
        let object = self.get_object_ref(key)?;
        match object.try_into() {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(format!("{e}")))
        }
    }

    pub fn get_optional<'a, T: 'a, E>(&'a self, key: impl AsRef<str>) -> Result<Option<T>> where E: std::error::Error, T: TryFrom<&'a Value, Error = E> {
        if let Ok(object) = self.get_object_ref(key) {
            if object.is_null() {
                Ok(None)
            } else {
                match object.try_into() {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => Err(Error::new(format!("{e}")))
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_object_ref(&self, key: impl AsRef<str>) -> Result<&Value> {
        if let Some(object) = self.inner.map.get(key.as_ref()) {
            Ok(object)
        } else {
            Err(Error::new(format!("argument '{}' is not present", key.as_ref())))
        }
    }

    pub fn get_object(&self, key: impl AsRef<str>) -> Result<Value> {
        self.get_object_ref(key).map(|o| o.clone())
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<String, Value> {
        self.inner.map.iter()
    }
}

impl Default for Arguments {

    fn default() -> Self {
        Arguments {
            inner: Arc::new(ArgumentsInner {
                map: btreemap!{}
            })
        }
    }
}

struct ArgumentsInner {
    map: BTreeMap<String, Value>
}