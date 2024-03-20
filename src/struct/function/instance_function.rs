use educe::Educe;
use std::sync::Arc;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;
use crate::value::Value;

#[derive(Educe, Serialize)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub body: Arc<dyn Function>,
}

pub trait Function: Send + Sync {
    fn call(&self, this: Value, arguments: Arguments) -> Result<Value>;
}

impl<F> Function for F where
    F: Fn(Value, Arguments) -> Result<Value> + Send + Sync {
    fn call(&self, this: Value, arguments: Arguments) -> Result<Value> {
        self(this, arguments)
    }
}

