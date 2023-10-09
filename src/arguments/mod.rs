use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::error::Error;
use crate::object::Object;
use crate::result::Result;

pub struct Arguments {
    inner: HashMap<String, Object>
}

impl Debug for Arguments {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Arguments");
        for (k, v) in &self.inner {
            debug_struct.field(k.as_str(), &v);
        }
        debug_struct.finish()
    }
}

impl Arguments {

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn has(&self, key: impl AsRef<str>) -> bool {
        self.inner.contains_key(key.as_ref())
    }

    pub fn get<'a, T: 'a>(&'a self, key: impl AsRef<str>) -> Result<T> where T: TryFrom<&'a Object, Error = Error> {
        if let Some(object) = self.inner.get(key.as_ref()) {
            object.try_into()
        } else {
            Err(Error::new(format!("argument '{}' is not present", key.as_ref())))
        }
    }
}