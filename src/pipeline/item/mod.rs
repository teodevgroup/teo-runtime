use std::future::Future;
use std::sync::Arc;
use educe::Educe;
use crate::arguments::Arguments;
use crate::pipeline::ctx::Ctx;
use teo_result::Result;
use futures_util::future::BoxFuture;
use serde::Serialize;
use crate::object::Object;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
pub struct Item {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
}

pub trait Call: Send + Sync {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, Result<Object>>;
}

impl<F, Fut> Call for F where
        F: Fn(Arguments, Ctx) -> Fut + Sync + Send,
        Fut: Future<Output = Result<Object>> + Send + 'static {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, Result<Object>> {
        Box::pin(self(args, ctx))
    }
}

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct BoundedItem {
    pub path: Vec<String>,
    pub arguments: Arguments,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
}

impl BoundedItem {

    pub(crate) async fn call(&self, args: Arguments, ctx: Ctx) -> Result<Object> {
        self.call.call(args, ctx).await
    }
}