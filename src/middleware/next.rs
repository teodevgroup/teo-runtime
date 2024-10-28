use std::future::Future;
use futures_util::future::BoxFuture;
use crate::request::Request;
use crate::response::Response;

pub trait Next: Send + Sync {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<F, Fut> Next for F where
    F: Fn(Request) -> Fut + Sync + Send,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        Box::pin(self(ctx))
    }
}
