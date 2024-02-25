use std::future::Future;
use futures_util::future::BoxFuture;
use crate::middleware::next::Next;
use crate::request::ctx::Ctx;
use crate::request::ctx::extract::ExtractFromRequestCtx;
use crate::response::Response;

pub trait Middleware: Send + Sync {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(Ctx, &'static dyn Next) -> Fut + Sync + Send,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
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

pub trait MiddlewareArgument<A>: Send + Sync + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<A0, F, Fut> MiddlewareArgument<(A0,)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value: A0 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(value, next))
    }
}

impl<A0, A1, F, Fut> MiddlewareArgument<(A0, A1)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    A1: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, A1, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequestCtx::extract(&ctx);
        let a1: A1 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(a0, a1, next))
    }
}

impl<A0, A1, A2, F, Fut> MiddlewareArgument<(A0, A1, A2)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    A1: ExtractFromRequestCtx + Send + Sync,
    A2: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, A1, A2, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequestCtx::extract(&ctx);
        let a1: A1 = ExtractFromRequestCtx::extract(&ctx);
        let a2: A2 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(a0, a1, a2, next))
    }
}

impl<A0, A1, A2, A3, F, Fut> MiddlewareArgument<(A0, A1, A2, A3)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    A1: ExtractFromRequestCtx + Send + Sync,
    A2: ExtractFromRequestCtx + Send + Sync,
    A3: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequestCtx::extract(&ctx);
        let a1: A1 = ExtractFromRequestCtx::extract(&ctx);
        let a2: A2 = ExtractFromRequestCtx::extract(&ctx);
        let a3: A3 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(a0, a1, a2, a3, next))
    }
}

pub fn middleware_wrap_fn<T, F>(call: F) -> &'static dyn Middleware where
    T: Send + Sync + 'static,
    F: MiddlewareArgument<T> + 'static
{
    let wrap_call = Box::leak(Box::new(call));
    Box::leak(Box::new(|ctx: Ctx, next: &'static dyn Next| async {
        wrap_call.call(ctx, next).await
    }))
}