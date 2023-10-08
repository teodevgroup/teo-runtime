use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::object::Object;

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

    pub fn has(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    // pub fn get
}