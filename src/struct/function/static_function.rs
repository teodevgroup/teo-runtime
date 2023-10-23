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

pub trait StaticFunction: Send + Sync {
    fn call(&self, arguments: Arguments) -> Result<Object>;
}

impl<F, Fut> StaticFunction for F where
    F: Fn(Arguments) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Object>> + Send + 'static {
    fn call(&self, arguments: Arguments) -> Result<Object> {
        self(arguments)
    }
}

