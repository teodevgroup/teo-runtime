use std::future::Future;
use futures_util::future::BoxFuture;
use crate::request::ctx::Ctx;
use crate::response::Response;
use teo_result::Result;
use crate::response::error::IntoResponseWithPathedError;

pub trait Next: Send + Sync {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>>;
}

impl<F, Fut> Next for F where
    F: Fn(Ctx) -> Fut + Sync + Send,
    Fut: Future<Output = crate::path::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>> {
        Box::pin(self(ctx))
    }
}