use std::future::Future;
use futures_util::future::BoxFuture;
use crate::value::Value;
use crate::pipeline::Ctx;
use crate::pipeline::item::templates::validator::ValidatorResult;
use teo_result::Error;
use crate::pipeline::ctx::extract::ExtractFromPipelineCtx;

pub trait Compare<A, O: Into<ValidatorResult>, E: Into<Error> + std::error::Error>: Send + Sync + Clone + 'static {
    fn call(&self, old: Value, new: Value, ctx: Ctx) -> BoxFuture<'static, O>;
}

impl<V1, V2, O, F, Fut, E> Compare<(V1, V2), O, E> for F where
    V1: TryFrom<Value, Error=E> + Send + Sync,
    V2: TryFrom<Value, Error=E> + Send + Sync,
    O: Into<ValidatorResult> + Send + Sync,
    F: Fn(V1, V2) -> Fut + Sync + Send + Clone + 'static,
    E: Into<Error> + std::error::Error,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, _ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        Box::pin(self(old, new))
    }
}

impl<V1, V2, A1, O, F, E, Fut> Compare<(V1, V2, A1), O, E> for F where
    V1: TryFrom<Value, Error=E> + Send + Sync,
    V2: TryFrom<Value, Error=E> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    O: Into<ValidatorResult> + Send + Sync,
    F: Fn(V1, V2, A1) -> Fut + Sync + Send + Clone + 'static,
    E: Into<Error> + std::error::Error,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        let a1 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(old, new, a1))
    }
}

impl<V1, V2, A1, A2, O, F, E, Fut> Compare<(V1, V2, A1, A2), O, E> for F where
    V1: TryFrom<Value, Error=E> + Send + Sync,
    V2: TryFrom<Value, Error=E> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    O: Into<ValidatorResult> + Send + Sync,
    F: Fn(V1, V2, A1, A2) -> Fut + Sync + Send + Clone + 'static,
    E: Into<Error> + std::error::Error,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        let a1 = ExtractFromPipelineCtx::extract(&ctx);
        let a2 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(old, new, a1, a2))
    }
}

impl<V1, V2, A1, A2, A3, O, F, E, Fut> Compare<(V1, V2, A1, A2, A3), O, E> for F where
    V1: TryFrom<Value, Error=E> + Send + Sync,
    V2: TryFrom<Value, Error=E> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    O: Into<ValidatorResult> + Send + Sync,
    F: Fn(V1, V2, A1, A2, A3) -> Fut + Sync + Send + Clone + 'static,
    E: Into<Error> + std::error::Error,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        let a1 = ExtractFromPipelineCtx::extract(&ctx);
        let a2 = ExtractFromPipelineCtx::extract(&ctx);
        let a3 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(old, new, a1, a2, a3))
    }
}