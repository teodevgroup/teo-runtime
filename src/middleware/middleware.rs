use std::future::Future;
use futures_util::future::BoxFuture;
use crate::middleware::next::Next;
use crate::request::ctx::Ctx;
use crate::response::Response;
use teo_result::Result;

pub trait Middleware: Send + Sync {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, Result<Response>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(Ctx, &'static dyn Next) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, Result<Response>> {
        Box::pin(self(ctx, next))
    }
}

pub(crate) fn empty_middleware() -> &'static dyn Middleware {
    Box::leak(Box::new(|ctx: Ctx, next: &'static dyn Next| async move {
        next.call(ctx).await
    }))
}

pub(crate) fn combine_middleware(middlewares: Vec<&'static dyn Middleware>) -> &'static dyn Middleware {
    match middlewares.len() {
        0 => empty_middleware(),
        1 => *middlewares.first().unwrap(),
        2 => join_middleware(*middlewares.get(1).unwrap(), *middlewares.get(0).unwrap()),
        _ => {
            let inner_most = *middlewares.first().unwrap();
            let mut result = join_middleware(*middlewares.get(1).unwrap(), inner_most);
            for (index, middleware) in middlewares.iter().enumerate() {
                if index >= 2 {
                    result = join_middleware(*middleware, result);
                }
            }
            result
        }
    }
}

fn join_middleware(outer: &'static dyn Middleware, inner: &'static dyn Middleware) -> &'static dyn Middleware {
    return Box::leak(Box::new(move |ctx: Ctx, next: &'static dyn Next| async move {
        outer.call(ctx, Box::leak(Box::new(move |ctx: Ctx| async move {
            inner.call(ctx, next).await
        }))).await
    }))
}