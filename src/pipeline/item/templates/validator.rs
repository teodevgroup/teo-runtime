use std::future::Future;
use futures_util::future::BoxFuture;
use self::Validity::*;
use teo_result::{Error, Result};
use crate::pipeline::Ctx;
use crate::pipeline::ctx::extract::ExtractFromPipelineCtx;
use crate::value::Value;

#[derive(Clone)]
pub enum Validity {
    Valid,
    Invalid(String)
}

impl Validity {
    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Valid => true,
            _ => false,
        }
    }

    pub(crate) fn invalid_reason(&self) -> Option<&str> {
        match self {
            Invalid(reason) => Some(&reason),
            _ => None,
        }
    }
}

impl From<&str> for Validity {
    fn from(reason: &str) -> Self {
        Invalid(reason.to_string())
    }
}

impl From<String> for Validity {
    fn from(reason: String) -> Self {
        Invalid(reason)
    }
}

impl From<bool> for Validity {
    fn from(valid: bool) -> Self {
        match valid {
            true => Valid,
            false => Invalid("value is invalid".to_owned())
        }
    }
}

impl From<()> for Validity {
    fn from(_: ()) -> Self {
        Valid
    }
}

impl From<Option<String>> for Validity {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => Invalid(v),
            None => Valid,
        }
    }
}

impl From<Option<&str>> for Validity {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(v) => Invalid(v.to_owned()),
            None => Valid,
        }
    }
}

pub enum ValidatorResult {
    Validity(Validity),
    Result(Result<Validity>),
}

impl<T> From<T> for ValidatorResult where T: Into<Validity> {
    fn from(value: T) -> Self {
        ValidatorResult::Validity(value.into())
    }
}

impl<T, U> From<std::result::Result<T, U>> for ValidatorResult where T: Into<Validity>, U: Into<Error> {
    fn from(value: std::result::Result<T, U>) -> Self {
        match value {
            Ok(t) => ValidatorResult::Result(Ok(t.into())),
            Err(e) => ValidatorResult::Result(Err(e.into())),
        }
    }
}

pub trait Validator<A, O: Into<ValidatorResult>>: Send + Sync + Clone + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O>;
}

impl<A0, O, F, Fut, E> Validator<(A0,), O> for F where
    A0: TryFrom<Value, Error=E> + Send + Sync,
    Error: From<E>,
    E: std::error::Error,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<ValidatorResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, Fut, E> Validator<(A0, A1), O> for F where
    A0: TryFrom<Value, Error=E> + Send + Sync,
    Error: From<E>,
    E: std::error::Error,
    A1: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + Clone + 'static,
    O: Into<ValidatorResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1))
    }
}

impl<A0, A1, A2, O, F, Fut, E> Validator<(A0, A1, A2), O> for F where
    A0: TryFrom<Value, Error=E> + Send + Sync,
    Error: From<E>,
    E: std::error::Error,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + Clone + 'static,
    O: Into<ValidatorResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2))
    }
}

impl<A0, A1, A2, A3, O, F, Fut, E> Validator<(A0, A1, A2, A3), O> for F where
    A0: TryFrom<Value, Error=E> + Send + Sync,
    Error: From<E>,
    E: std::error::Error,
    A1: ExtractFromPipelineCtx + Send + Sync,
    A2: ExtractFromPipelineCtx + Send + Sync,
    A3: ExtractFromPipelineCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + Clone + 'static,
    O: Into<ValidatorResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, O> {
        let value: A0 = ctx.value().clone().try_into().unwrap();
        let arg1: A1 = ExtractFromPipelineCtx::extract(&ctx);
        let arg2: A2 = ExtractFromPipelineCtx::extract(&ctx);
        let arg3: A3 = ExtractFromPipelineCtx::extract(&ctx);
        Box::pin(self(value, arg1, arg2, arg3))
    }
}