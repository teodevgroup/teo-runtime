use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use history_box::HistoryBox;
use maplit::btreemap;
use teo_result::{Result, Error};
use crate::Value;

#[derive(Default, Clone)]
pub struct LocalValues {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    map: HistoryBox<BTreeMap<String, HistoryBox<Value>>>,
}

impl LocalValues {

    #[inline]
    pub fn new() -> LocalValues {
        Self {
            inner: Arc::new(Inner {
                map: HistoryBox::new_with(btreemap! {}),
            })
        }
    }

    fn map_mut(&self) -> &mut BTreeMap<String, HistoryBox<Value>> {
        unsafe { self.inner.map.get_mut().unwrap() }
    }

    fn map_immut(&self) -> &BTreeMap<String, HistoryBox<Value>> {
        unsafe { self.inner.map.get().unwrap() }
    }

    pub fn insert<T: 'static + Into<Value>>(&self, key: impl Into<String>, val: T) {
        let key = key.into();
        let contains = self.map_immut().contains_key(&key);
        if contains {
            self.map_mut().get_mut(&key).unwrap().set(val.into());
        } else {
            self.map_mut().insert(key, HistoryBox::new_with(val.into()));
        }
    }

    pub fn get<'a, T>(&'a self, key: &str) -> Result<T> where T: 'a + TryFrom<&'a Value>, Error: From<T::Error> {
        match self.map_immut().get(key) {
            None => Err(Error::new(format!("value not found for key: {}", key))),
            Some(v) => {
                let v = T::try_from(v.get().unwrap())?;
                Ok(v)
            }
        }
    }

    pub fn get_mut(&self, key: &str) -> Result<&mut Value> {
        match self.map_immut().get(key) {
            None => Err(Error::new(format!("value not found for key: {}", key))),
            Some(v) => Ok(unsafe { v.get_mut() }.unwrap())
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.map_immut().contains_key(key)
    }

    pub fn remove(&self, key: &str) {
        self.map_mut().remove(key);
    }

    pub fn len(&self) -> usize {
        self.map_immut().len()
    }

    pub fn clear(&self) {
        self.map_mut().clear()
    }
}

impl Debug for LocalValues {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("LocalValues");
        for (k, v) in self.map_immut() {
            debug_struct.field(&k, &v);
        }
        debug_struct.finish()
    }
}

unsafe impl Send for LocalValues { }
unsafe impl Sync for LocalValues { }
