use std::cell::Cell;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use educe::Educe;
use crate::app::cleanup::Cleanup;
use crate::app::entrance::Entrance;
use crate::app::runtime_version::RuntimeVersion;

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct AppData {
    inner: Arc<Inner>,
}

#[derive(Educe)]
#[educe(Debug)]
struct Inner {
    entrance: Entrance,
    runtime_version: RuntimeVersion,
    /// This is designed for Node.js and Python
    /// A place to store dynamic runtime classes
    #[educe(Debug(ignore))]
    dynamic_classes_pointer: Cell<* mut ()>,
    /// This is designed for Node.js and Python
    #[educe(Debug(ignore))]
    dynamic_classes_clean_up: Arc<Mutex<Option<Arc<dyn Cleanup>>>>,
}

impl AppData {

    pub fn new(entrance: Entrance, runtime_version: RuntimeVersion) -> Self {
        Self {
            inner: Arc::new(Inner {
                entrance,
                runtime_version,
                dynamic_classes_pointer: Cell::new(null_mut()),
                dynamic_classes_clean_up: Arc::new(Mutex::new(None)),
            })
        }
    }

    pub fn entrance(&self) -> &Entrance {
        &self.inner.entrance
    }

    pub fn runtime_version(&self) -> &RuntimeVersion {
        &self.inner.runtime_version
    }


    pub fn dynamic_classes_pointer(&self) -> * mut () {
        self.inner.dynamic_classes_pointer.get()
    }

    pub fn set_dynamic_classes_pointer(&self, pointer: * mut ()) {
        self.inner.dynamic_classes_pointer.set(pointer);
    }

    pub fn dynamic_classes_clean_up(&self) -> Option<Arc<dyn Cleanup>> {
        self.inner.dynamic_classes_clean_up.lock().unwrap().clone()
    }

    pub fn set_dynamic_classes_clean_up(&self, clean_up: Arc<dyn Cleanup>) {
        *self.inner.dynamic_classes_clean_up.lock().unwrap() = Some(clean_up);
    }
}

unsafe impl Send for AppData { }
unsafe impl Sync for AppData { }