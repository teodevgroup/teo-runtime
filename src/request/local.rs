use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct Data {
    map: BTreeMap<String, Box<dyn Any + Send + Sync>>,
}

impl Data {

    #[inline]
    pub fn new() -> Data {
        Self {
            map: BTreeMap::default(),
        }
    }

    pub fn insert<T: 'static + Send + Sync>(&mut self, key: impl Into<String>, val: T) {
        self.map.insert(key.into(), Box::new(val));
    }

    pub fn get<T: 'static + Send>(&self, key: &str) -> Option<&T> {
        self.map.get(key).and_then(|boxed| boxed.downcast_ref())
    }

    pub fn get_mut<T: 'static + Send>(&mut self, key: &str) -> Option<&mut T> {
        self.map.get_mut(key).and_then(|boxed| boxed.downcast_mut())
    }

    pub fn contains<T: 'static + Send>(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove<T: 'static + Send>(&mut self, key: &str) -> Option<&T> {
        self.map.remove(key).and_then(|boxed| downcast_owned(boxed))
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }
}

impl Debug for Data {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("LocalData");
        for (k, v) in &self.map {
            debug_struct.field(&k, &v);
        }
        debug_struct.finish()
    }
}

fn downcast_owned<T: 'static>(boxed: Box<dyn Any>) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}