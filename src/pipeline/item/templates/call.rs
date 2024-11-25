use std::future::Future;
use futures_util::future::BoxFuture;
use crate::pipeline::Ctx;
use crate::Value;

pub trait Call: Send + Sync {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, teo_result::Result<Value>>;
}

impl<F, Fut> Call for F where
    F: Fn(Ctx) -> Fut + Sync + Send,
    Fut: Future<Output = teo_result::Result<Value>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, teo_result::Result<Value>> {
        Box::pin(self(ctx))
    }
}
