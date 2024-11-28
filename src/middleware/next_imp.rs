use std::future::Future;
use futures_util::future::BoxFuture;
use crate::request::Request;
use crate::response::Response;
use teo_result::Result;

pub trait NextImp: Send + Sync {
    fn call(&self, request: Request) -> BoxFuture<'static, Result<Response>>;
}

impl<F, Fut> NextImp for F where
    F: Fn(Request) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'static {
    fn call(&self, ctx: Request) -> BoxFuture<'static, Result<Response>> {
        Box::pin(self(ctx))
    }
}
