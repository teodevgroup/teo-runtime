use std::future::Future;
use std::sync::Arc;
use educe::Educe;
use futures_util::future::BoxFuture;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::Value;

pub trait Call: Send + Sync {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, teo_result::Result<Value>>;
}

impl<F, Fut> Call for F where
    F: Fn(Arguments, Ctx) -> Fut + Sync + Send,
    Fut: Future<Output =teo_result::Result<Value>> + Send + 'static {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, teo_result::Result<Value>> {
        Box::pin(self(args, ctx))
    }
}

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Item {
    inner: Arc<Inner>
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct Inner {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
}

impl Item {

    pub fn new(path: Vec<String>, call: Arc<dyn Call>) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                call
            })
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn call(&self) -> Arc<dyn Call> {
        self.inner.call.clone()
    }
}