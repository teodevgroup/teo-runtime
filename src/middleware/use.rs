use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::middleware::creator::Creator;

#[derive(Debug, Clone)]
pub struct Use {
    inner: Arc<Inner>,
}

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
struct Inner {
    path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    creator: Arc<dyn Creator>,
    arguments: Arguments,
}

impl Use {
    pub fn new(path: Vec<String>, creator: Arc<dyn Creator>, arguments: Arguments) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                creator,
                arguments,
            }),
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn creator(&self) -> &Arc<dyn Creator> {
        &self.inner.creator
    }

    pub fn arguments(&self) -> &Arguments {
        &self.inner.arguments
    }
}

unsafe impl Send for Use { }
unsafe impl Sync for Use { }