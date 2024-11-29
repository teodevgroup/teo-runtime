use futures_util::future::BoxFuture;
use std::future::Future;
use teo_result::{Error, Result};
use crate::pipeline::Ctx;
use crate::value::Value;
use super::super::super::ctx::extract::ExtractFromPipelineCtx;

pub enum TransformerResult<T> where T: Into<Value> {
    Object(T),
    Result(Result<T>),
}

impl<T> From<T> for TransformerResult<T> where T: Into<Value> {
    fn from(value: T) -> Self {
        TransformerResult::Object(value)
    }
}

impl<T, U> From<std::result::Result<T, U>> for TransformerResult<T> where T: Into<Value>, U: Into<Error> {
    fn from(result: std::result::Result<T, U>) -> Self {
        match result {
            Ok(t) => TransformerResult::Result(Ok(t)),
            Err(err) => TransformerResult::Result(Err(err.into())),
        }
    }
}

pub trait Transformer<A, O: Into<Value>, R: Into<TransformerResult<O>>>: Send + Sync + Clone + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R>;
}

impl<O, F, R, Fut> Transformer<(), O, R> for F where
    F: Fn() -> Fut + Sync + Send + Clone + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformerResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, _: Ctx) -> BoxFuture<'static, R> {
        Box::pin(self())
    }
}

impl<A0, O, F, R, Fut> Transformer<(A0,), O, R> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformerResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, R, Fut> Transformer<(A0, A1), O, R> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + Clone + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformerResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1))
    }
}

impl<A0, A1, A2, O, F, R, Fut> Transformer<(A0, A1, A2), O, R> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + Clone + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformerResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2))
    }
}

impl<A0, A1, A2, A3, O, F, R, Fut> Transformer<(A0, A1, A2, A3), O, R> for F where
    A0: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + Clone + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformerResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        let arg3: A3 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2, arg3))
    }
}