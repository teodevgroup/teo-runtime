use futures_util::future::BoxFuture;
use std::future::Future;
use teo_result::{Error, Result};
use crate::object::Object;
use crate::pipeline::Ctx;
use super::super::ctx::extract::ExtractFromPipelineCtx;

pub enum TransformResult<T> where T: Into<Object> {
    Object(T),
    Result(Result<T>),
}

impl<T> From<T> for TransformResult<T> where T: Into<Object> {
    fn from(value: T) -> Self {
        TransformResult::Object(value)
    }
}

impl<T, U> From<std::result::Result<T, U>> for TransformResult<T> where T: Into<Object>, U: Into<Error> {
    fn from(result: std::result::Result<T, U>) -> Self {
        match result {
            Ok(t) => TransformResult::Result(Ok(t)),
            Err(err) => TransformResult::Result(Err(err.into())),
        }
    }
}

pub trait TransformArgument<A, O: Into<Object>, R: Into<TransformResult<O>>>: Send + Sync + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R>;
}

impl<A0, O, F, R, Fut> TransformArgument<(A0,), O, R> for F where
    A0: TryFrom<Object, Error=Error> + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<Object> + Sync + Send,
    R: Into<TransformResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, R, Fut> TransformArgument<(A0, A1), O, R> for F where
    A0: TryFrom<Object, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + 'static,
    O: Into<Object> + Sync + Send,
    R: Into<TransformResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1))
    }
}

impl<A0, A1, A2, O, F, R, Fut> TransformArgument<(A0, A1, A2), O, R> for F where
    A0: TryFrom<Object, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + 'static,
    O: Into<Object> + Sync + Send,
    R: Into<TransformResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2))
    }
}

impl<A0, A1, A2, A3, O, F, R, Fut> TransformArgument<(A0, A1, A2, A3), O, R> for F where
    A0: TryFrom<Object, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + 'static,
    O: Into<Object> + Sync + Send,
    R: Into<TransformResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, R> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        let arg3: A3 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2, arg3))
    }
}