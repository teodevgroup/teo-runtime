use std::any::Any;
use std::sync::Arc;
use deferred_box::DeferredBox;
use educe::Educe;
use teo_result::{Result, Error};
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
    dynamic_classes: DeferredBox<Arc<dyn Any>>,
}

impl AppData {

    pub fn new(entrance: Entrance, runtime_version: RuntimeVersion) -> Self {
        Self {
            inner: Arc::new(Inner {
                entrance,
                runtime_version,
                dynamic_classes: DeferredBox::new(),
            })
        }
    }

    pub fn entrance(&self) -> &Entrance {
        &self.inner.entrance
    }

    pub fn runtime_version(&self) -> &RuntimeVersion {
        &self.inner.runtime_version
    }

    pub fn dynamic_classes(&self) -> Result<&dyn Any> {
        match self.inner.dynamic_classes.get() {
            Some(dynamic_classes) => Ok(dynamic_classes.as_ref()),
            None => Err(Error::new("Dynamic classes is accessed while not set")),
        }
    }

    pub fn set_dynamic_classes(&self, dynamic_classes: Arc<dyn Any>) -> Result<()> {
        match self.inner.dynamic_classes.set(dynamic_classes) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new("Dynamic classes have been set")),
        }
    }
}

unsafe impl Send for AppData { }
unsafe impl Sync for AppData { }