use educe::Educe;
use std::future::Future;
use std::sync::Arc;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::object::Object;
use teo_result::Result;

#[derive(Educe, Serialize)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub body: Arc<dyn Function>,
}

pub trait Function: Send + Sync {
    fn call(&self, this: Object, arguments: Arguments) -> Result<Object>;
}

impl<F> Function for F where
    F: Fn(Object, Arguments) -> Result<Object> + Send + Sync {
    fn call(&self, this: Object, arguments: Arguments) -> Result<Object> {
        self(this, arguments)
    }
}

