use std::future::Future;
use futures_util::future::BoxFuture;
use crate::arguments::Arguments;
use crate::middleware::middleware::Middleware;
use teo_result::Result;
use crate::middleware::MiddlewareImpl;

pub trait Creator: Send + Sync {
    fn call(&self, arguments: Arguments) -> BoxFuture<'static, Result<MiddlewareImpl>>;
}

impl<F, Fut> Creator for F where
    F: Fn(Arguments) -> Fut + Send + Sync,
    Fut: Future<Output = Result<MiddlewareImpl>> + Send + 'static {
    fn call(&self, args: Arguments) -> BoxFuture<'static, Result<MiddlewareImpl>> {
        Box::pin(self(args))
    }
}