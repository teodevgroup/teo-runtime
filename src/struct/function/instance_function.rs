use educe::Educe;
use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::arguments::Arguments;
use crate::object::Object;
use teo_result::Result;

#[derive(Educe)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))]
    pub body: Arc<dyn Function>,
}

pub trait Function: Send + Sync {
    fn call(&self, this: Object, arguments: Arguments) -> BoxFuture<'static, Result<Object>>;
}

impl<F, Fut> Function for F where
    F: Fn(Object, Arguments) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Object>> + Send + 'static {
    fn call(&self, this: Object, arguments: Arguments) -> BoxFuture<'static, Result<Object>> {
        Box::pin(self(this, arguments))
    }
}

