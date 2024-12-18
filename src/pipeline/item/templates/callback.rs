use std::future::Future;
use futures_util::future::BoxFuture;
use teo_result::{Error, Result};
use crate::pipeline::Ctx;
use crate::pipeline::ctx::extract::ExtractFromPipelineCtx;
use crate::Value;

pub enum CallbackResult {
    Result(Result<()>)
}

impl From<()> for CallbackResult {
    fn from(_: ()) -> Self {
        CallbackResult::Result(Ok(()))
    }
}

impl From<Result<()>> for CallbackResult {
    fn from(result: Result<()>) -> Self {
        CallbackResult::Result(result)
    }
}

pub trait Callback<A, O: Into<CallbackResult>>: Send + Sync + Clone + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O>;
}

impl<O, F, Fut> Callback<(), O> for F where
    F: Fn() -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, _: Ctx) -> BoxFuture<'static, O> {
        Box::pin(self())
    }
}

impl<A0, O, F, Fut> Callback<(A0,), O> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, Fut> Callback<(A0, A1), O> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1))
    }
}

impl<A0, A1, A2, O, F, Fut> Callback<(A0, A1, A2), O> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2))
    }
}

impl<A0, A1, A2, A3, O, F, Fut> Callback<(A0, A1, A2, A3), O> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        let arg3: A3 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2, arg3))
    }
}

impl<A0, A1, A2, A3, A4, O, F, Fut> Callback<(A0, A1, A2, A3, A4), O> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    A4: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3, A4) -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        let arg3: A3 = ExtractFromPipelineCtx::extract(&ctx);
        let arg4: A4 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2, arg3, arg4))
    }
}