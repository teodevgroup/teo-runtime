use std::future::Future;
use futures_util::future::BoxFuture;
use crate::request::ctx::Ctx;
use crate::response::Response;
use async_trait::async_trait;

#[async_trait]
pub trait Next: Send + Sync {
    async fn call(&self, ctx: Ctx) -> crate::path::Result<Response>;
}

#[async_trait]
impl<F, Fut> Next for F where
    F: Fn(Ctx) -> Fut + Send + Sync,
    Fut: Future<Output = crate::path::Result<Response>> + Send + 'static {
    async fn call(&self, ctx: Ctx) -> crate::path::Result<Response> {
        self(ctx).await
    }
}
