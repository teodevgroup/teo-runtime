use std::future::Future;
use futures_util::future::BoxFuture;
use crate::arguments::Arguments;
use teo_result::Result;
use crate::middleware::Middleware;

pub trait Creator: Send + Sync {
    fn call(&self, arguments: Arguments) -> BoxFuture<'static, Result<Middleware>>;
}

impl<F, Fut> Creator for F where
    F: Fn(Arguments) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Middleware>> + Send + 'static {
    fn call(&self, args: Arguments) -> BoxFuture<'static, Result<Middleware>> {
        Box::pin(self(args))
    }
}