use std::future::Future;
use futures_util::future::BoxFuture;
use crate::request::ctx::Ctx;
use crate::response::Response;
use crate::result::Result;

pub trait Next: Send + Sync {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<Response>>;
}

impl<F, Fut> Next for F where
    F: Fn(Ctx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'static {
    fn call(&self, req_ctx: Ctx) -> BoxFuture<'static, Result<Response>> {
        Box::pin(self(req_ctx))
    }
}