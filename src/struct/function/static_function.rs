use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;
use crate::value::Value;

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub body: Arc<dyn StaticFunction>,
}

pub trait StaticFunction: Send + Sync {
    fn call(&self, arguments: Arguments) -> Result<Value>;
}

impl<F> StaticFunction for F where
    F: Fn(Arguments) -> Result<Value> + Send + Sync {
    fn call(&self, arguments: Arguments) -> Result<Value> {
        self(arguments)
    }
}

