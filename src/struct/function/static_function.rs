use std::future::Future;
use std::sync::Arc;
use educe::Educe;
use futures_util::future::BoxFuture;
use crate::arguments::Arguments;
use crate::object::Object;
use teo_result::Result;

#[derive(Educe)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))]
    pub body: Arc<dyn StaticFunction>,
}

pub trait StaticFunction {
    fn call(&self, arguments: Arguments) -> Result<Object>;
}

impl<F> StaticFunction for F where
    F: Fn(Arguments) -> Result<Object> {
    fn call(&self, arguments: Arguments) -> Result<Object> {
        self(arguments)
    }
}

