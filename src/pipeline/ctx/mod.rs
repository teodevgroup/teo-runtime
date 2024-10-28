pub mod extract;

use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use key_path::KeyPath;
use crate::model;
use crate::pipeline::pipeline::Pipeline;
use crate::request;
use teo_result::{Result, ResultExt};
use crate::action::Action;
use crate::connection::transaction;
use teo_result::Error;
use crate::value::Value;

#[derive(Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>,
}

unsafe impl Send for Ctx { }
unsafe impl Sync for Ctx { }

#[derive(Debug)]
struct CtxInner {
    value: Value,
    object: model::Object,
    path: KeyPath,
    action: Action,
    transaction_ctx: transaction::Ctx,
    request: Option<request::Request>,
}

impl Ctx {

    pub fn new(value: Value, object: model::Object, path: KeyPath, action: Action, transaction_ctx: transaction::Ctx, request: Option<request::Request>) -> Self {
        Self {
            inner: Arc::new(CtxInner { value, object, path, action, transaction_ctx, request })
        }
    }

    pub fn value(&self) -> &Value {
        &self.inner.value
    }

    pub fn object(&self) -> &model::Object {
        &self.inner.object
    }

    pub fn path(&self) -> &KeyPath {
        &self.inner.path
    }

    pub fn action(&self) -> Action {
        self.inner.action
    }

    pub fn transaction_ctx(&self) -> transaction::Ctx {
        self.inner.transaction_ctx.clone()
    }

    pub fn request(&self) -> Option<request::Request> {
        self.inner.request.clone()
    }

    pub async fn resolve_pipeline<T, E>(&self, object: Value) -> Result<T> where T: TryFrom<Value, Error=E>, Error: From<E> {
        if let Some(pipeline) = object.as_pipeline() {
            self.run_pipeline(pipeline).await
        } else {
            Ok(object.try_into()?)
        }
    }

    pub async fn resolve_pipeline_with_err_prefix<T, E>(&self, object: Value, err_prefix: impl AsRef<str>) -> Result<T> where T: TryFrom<Value, Error=E>, Error: From<E> {
        if let Some(pipeline) = object.as_pipeline() {
            self.run_pipeline_with_err_prefix(pipeline, err_prefix).await
        } else {
            Ok(object.try_into_err_prefix(err_prefix)?)
        }
    }

    async fn run_pipeline_inner<T, E>(&self, pipeline: &Pipeline) -> Result<T> where T: TryFrom<Value, Error=E>, Error: From<E> {
        let mut ctx = self.clone();
        for item in &pipeline.items {
            ctx = ctx.alter_value(item.call(item.arguments.clone(), ctx.clone()).await?.cast(item.cast_output_type.as_ref(), self.transaction_ctx().namespace()));
        }
        Ok(ctx.value().clone().try_into()?)
    }

    pub async fn run_pipeline<T, E>(&self, pipeline: &Pipeline) -> Result<T> where T: TryFrom<Value, Error=E>, Error: From<E> {
        let result = self.run_pipeline_inner(pipeline).await;
        result.map_err(|e| e.pathed(self.path().to_string()))
    }

    pub async fn run_pipeline_ignore_return_value(&self, pipeline: &Pipeline) -> Result<()> {
        let _: Value = self.run_pipeline(pipeline).await?;
        Ok(())
    }

    pub async fn run_pipeline_with_err_prefix<T, E>(&self, pipeline: &Pipeline, err_prefix: impl AsRef<str>) -> Result<T> where T: TryFrom<Value, Error=E>, Error: From<E> {
        self.run_pipeline(pipeline).await.error_message_prefixed(err_prefix)
    }

    pub async fn run_pipeline_with_err_prefix_ignore_return_value(&self, pipeline: &Pipeline, err_prefix: impl AsRef<str>) -> Result<()> {
        let _: Value = self.run_pipeline_with_err_prefix(pipeline, err_prefix).await?;
        Ok(())
    }


    pub fn alter_value(&self, value: Value) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                value,
                object: self.inner.object.clone(),
                path: self.inner.path.clone(),
                action: self.inner.action,
                transaction_ctx: self.inner.transaction_ctx.clone(),
                request: self.inner.request.clone(),
            })
        }
    }
}

impl<'a> Debug for Ctx {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.inner.as_ref(), f)
    }
}