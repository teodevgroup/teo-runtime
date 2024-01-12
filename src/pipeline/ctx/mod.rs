pub mod extract;

use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use key_path::KeyPath;
use crate::object::Object;
use crate::model;
use crate::pipeline::pipeline::Pipeline;
use crate::request;
use teo_result::{Result, ResultExt};
use crate::action::Action;
use crate::connection::transaction;

#[derive(Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>,
}

unsafe impl Send for Ctx { }
unsafe impl Sync for Ctx { }

#[derive(Debug)]
struct CtxInner {
    value: Object,
    object: model::Object,
    path: KeyPath,
    action: Action,
    transaction_ctx: transaction::Ctx,
    request_ctx: Option<request::Ctx>,
}

impl Ctx {

    pub fn new(value: Object, object: model::Object, path: KeyPath, action: Action, transaction_ctx: transaction::Ctx, request_ctx: Option<request::Ctx>) -> Self {
        Self {
            inner: Arc::new(CtxInner { value, object, path, action, transaction_ctx, request_ctx })
        }
    }

    pub fn value(&self) -> &Object {
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

    pub fn request_ctx(&self) -> Option<request::Ctx> {
        self.inner.request_ctx.clone()
    }

    pub async fn resolve_pipeline(&self, object: Object, err_prefix: impl AsRef<str>) -> Result<Object> {
        if let Some(pipeline) = object.as_pipeline() {
            self.run_pipeline_with_err_prefix(pipeline, err_prefix).await
        } else {
            Ok(object)
        }
    }

    pub async fn run_pipeline_with_err_prefix(&self, pipeline: &Pipeline, err_prefix: impl AsRef<str>) -> Result<Object> {
        self.run_pipeline(pipeline).await.err_prefix(err_prefix)
    }

    pub async fn run_pipeline(&self, pipeline: &Pipeline) -> Result<Object> {
        let mut ctx = self.clone();
        for item in &pipeline.items {
            ctx = ctx.alter_value(item.call(item.arguments.clone(), ctx.clone()).await?.cast(item.cast_output_type.as_ref(), self.transaction_ctx().namespace()));
        }
        Ok(ctx.value().clone())
    }

    pub async fn run_pipeline_into_path_value_error(&self, pipeline: &Pipeline) -> crate::path::Result<Object> {
        match self.run_pipeline(pipeline).await {
            Ok(object) => Ok(object),
            Err(e) => Err(crate::path::Error::value_error(self.path().clone(), e.message))
        }
    }

    pub async fn run_pipeline_into_path_unauthorized_error(&self, pipeline: &Pipeline) -> crate::path::Result<Object> {
        match self.run_pipeline(pipeline).await {
            Ok(object) => Ok(object),
            Err(e) => Err(crate::path::Error::unauthorized_error(self.path().clone(), e.message))
        }
    }

    pub fn alter_value(&self, value: Object) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                value,
                object: self.inner.object.clone(),
                path: self.inner.path.clone(),
                action: self.inner.action,
                transaction_ctx: self.inner.transaction_ctx.clone(),
                request_ctx: self.inner.request_ctx.clone(),
            })
        }
    }
}

impl<'a> Debug for Ctx {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.inner.as_ref(), f)
    }
}