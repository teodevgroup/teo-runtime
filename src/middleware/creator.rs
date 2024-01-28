use std::future::Future;
use futures_util::future::BoxFuture;
use crate::arguments::Arguments;
use crate::middleware::middleware::Middleware;
use teo_result::Result;

pub trait Creator {
    fn call(&self, arguments: Arguments) -> BoxFuture<'static, Result<&'static dyn Middleware>>;
}

impl<F, Fut> Creator for F where
    F: Fn(Arguments) -> Fut,
    Fut: Future<Output = Result<&'static dyn Middleware>> + Send + 'static {
    fn call(&self, args: Arguments) -> BoxFuture<'static, Result<&'static dyn Middleware>> {
        Box::pin(self(args))
    }
}