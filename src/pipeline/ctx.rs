use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use key_path::KeyPath;
use teo_result::Error;
use crate::object::Object;
use crate::request::Request;
use crate::model;
use crate::pipeline::pipeline::Pipeline;
use crate::request;
use teo_result::{Result, ResultExt};

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
    //pub(crate) action: Action,
    //pub(crate) conn: Arc<dyn Connection>,
    request_ctx: Option<request::Ctx>,
}

impl Ctx {

    pub fn value(&self) -> &Object {
        &self.inner.value
    }

    pub fn object(&self) -> &model::Object {
        &self.inner.object
    }

    pub fn path(&self) -> &KeyPath {
        &self.inner.path
    }

    pub fn request_ctx(&self) -> Option<&request::Ctx> {
        self.inner.request_ctx.as_ref()
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
            ctx = ctx.alter_value(item.call(item.arguments.clone(), ctx.clone()).await?);
        }
        Ok(ctx.value().clone())
    }

    pub fn alter_value(&self, value: Object) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                value,
                object: self.inner.object.clone(),
                path: self.inner.path.clone(),
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