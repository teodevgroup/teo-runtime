use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use history_box::HistoryBox;

#[derive(Default)]
pub struct LocalObjects {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    map: HistoryBox<BTreeMap<String, HistoryBox<Box<dyn Any>>>>,
}

impl LocalObjects {

    #[inline]
    pub fn new() -> LocalObjects {
        Self {
            inner: Arc::new(Inner {
                map: HistoryBox::new_with(BTreeMap::new()),
            })
        }
    }

    fn map_mut(&self) -> &mut BTreeMap<String, HistoryBox<Box<dyn Any>>> {
        unsafe { self.inner.map.get_mut().unwrap() }
    }

    fn map_immut(&self) -> &BTreeMap<String, HistoryBox<Box<dyn Any>>> {
        unsafe { self.inner.map.get().unwrap() }
    }

    pub fn insert<T: 'static>(&self, key: impl Into<String>, val: T) {
        let key = key.into();
        let contains = self.map_immut().contains_key(&key);
        if contains {
            self.map_mut().get_mut(&key).unwrap().set(Box::new(val));
        } else {
            self.map_mut().insert(key, HistoryBox::new_with(Box::new(val)));
        }
    }

    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.map_immut().get(key).and_then(|boxed| boxed.get().unwrap().downcast_ref())
    }

    pub fn get_mut<T: 'static>(&self, key: &str) -> Option<&mut T> {
        self.map_immut().get(key).and_then(|boxed| unsafe { boxed.get_mut() }.unwrap().downcast_mut())
    }

    pub fn contains<T: 'static>(&self, key: &str) -> bool {
        self.map_immut().contains_key(key)
    }

    pub fn remove<T: 'static>(&self, key: &str) {
        self.map_mut().remove(key);
    }

    pub fn clear(&self) {
        self.map_mut().clear()
    }
}

impl Debug for LocalObjects {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("LocalObjects");
        for (k, v) in self.map_immut() {
            debug_struct.field(&k, &v);
        }
        debug_struct.finish()
    }
}

fn downcast_owned<T: 'static>(boxed: Box<dyn Any>) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}

unsafe impl Send for LocalObjects { }
unsafe impl Sync for LocalObjects { }