use std::future::Future;
use futures_util::future::BoxFuture;
use teo_teon::Value;
use crate::pipeline::Ctx;
use crate::pipeline::item::validator::ValidateResult;
use teo_result::Error;
use crate::arguments::Arguments;
use crate::pipeline::ctx::extract::ExtractFromPipelineCtx;

pub trait CompareArgument<A, O: Into<ValidateResult>>: Send + Sync + 'static {
    fn call(&self, old: Value, new: Value, args: Arguments, ctx: Ctx) -> BoxFuture<'static, O>;
}

impl<V1, V2, O, F, Fut> CompareArgument<(V1, V2), O> for F where
    V1: TryFrom<Value, Error=Error> + Send + Sync,
    V2: TryFrom<Value, Error=Error> + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, args: Arguments, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        Box::pin(self(old, new))
    }
}

impl<V1, V2, A1, O, F, Fut> CompareArgument<(V1, V2, A1), O> for F where
    V1: TryFrom<Value, Error=Error> + Send + Sync,
    V2: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2, A1) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, args: Arguments, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        let a1 = ExtractFromPipelineCtx::extract(&args, &ctx);
        Box::pin(self(old, new, a1))
    }
}

impl<V1, V2, A1, A2, O, F, Fut> CompareArgument<(V1, V2, A1, A2), O> for F where
    V1: TryFrom<Value, Error=Error> + Send + Sync,
    V2: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2, A1, A2) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, args: Arguments, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        let a1 = ExtractFromPipelineCtx::extract(&args, &ctx);
        let a2 = ExtractFromPipelineCtx::extract(&args, &ctx);
        Box::pin(self(old, new, a1, a2))
    }
}

impl<V1, V2, A1, A2, A3, O, F, Fut> CompareArgument<(V1, V2, A1, A2, A3), O> for F where
    V1: TryFrom<Value, Error=Error> + Send + Sync,
    V2: TryFrom<Value, Error=Error> + Send + Sync,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2, A1, A2, A3) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: Value, new: Value, args: Arguments, ctx: Ctx) -> BoxFuture<'static, O> {
        let old: V1 = old.try_into().unwrap();
        let new: V2 = new.try_into().unwrap();
        let a1 = ExtractFromPipelineCtx::extract(&args, &ctx);
        let a2 = ExtractFromPipelineCtx::extract(&args, &ctx);
        let a3 = ExtractFromPipelineCtx::extract(&args, &ctx);
        Box::pin(self(old, new, a1, a2, a3))
    }
}