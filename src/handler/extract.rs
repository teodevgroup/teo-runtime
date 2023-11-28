use std::future::Future;
use futures_util::future::BoxFuture;
use crate::middleware::next::Next;
use crate::request::Ctx;
use crate::response::Response;

pub trait ExtractFromRequestCtx {
    fn extract(ctx: &Ctx) -> Self;
}

pub trait HandlerCtxArgument<A>: Send + Sync + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>>;
}

// impl<A> Next for dyn HandlerCtxArgument<A> {
//     fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>> {
//         Box::pin(self(ctx))
//     }
// }

impl<A0, F, Fut> HandlerCtxArgument<(A0,)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = crate::path::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>> {
        let value: A0 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(value))
    }
}

impl<A0, A1, F, Fut> HandlerCtxArgument<(A0, A1)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    A1: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = crate::path::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>> {
        let value0: A0 = ExtractFromRequestCtx::extract(&ctx);
        let value1: A1 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(value0, value1))
    }
}

impl<A0, A1, A2, F, Fut> HandlerCtxArgument<(A0, A1, A2)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    A1: ExtractFromRequestCtx + Send + Sync,
    A2: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = crate::path::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>> {
        let value0: A0 = ExtractFromRequestCtx::extract(&ctx);
        let value1: A1 = ExtractFromRequestCtx::extract(&ctx);
        let value2: A2 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(value0, value1, value2))
    }
}

impl<A0, A1, A2, A3, F, Fut> HandlerCtxArgument<(A0, A1, A2, A3)> for F where
    A0: ExtractFromRequestCtx + Send + Sync,
    A1: ExtractFromRequestCtx + Send + Sync,
    A2: ExtractFromRequestCtx + Send + Sync,
    A3: ExtractFromRequestCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = crate::path::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, crate::path::Result<Response>> {
        let value0: A0 = ExtractFromRequestCtx::extract(&ctx);
        let value1: A1 = ExtractFromRequestCtx::extract(&ctx);
        let value2: A2 = ExtractFromRequestCtx::extract(&ctx);
        let value3: A3 = ExtractFromRequestCtx::extract(&ctx);
        Box::pin(self(value0, value1, value2, value3))
    }
}