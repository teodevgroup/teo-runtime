use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use key_path::KeyPath;
use crate::error::Error;
use crate::object::Object;
use crate::request::Request;
use crate::model;
use crate::result::{Result, ResultExt};

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
    request: Option<Request>,
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

    pub fn request(&self) -> Option<&Request> {
        self.inner.request.as_ref()
    }

    pub async fn resolve_pipeline(&self, object: Object, err_prefix: impl AsRef<str>) -> Result<Object> {
        if let Some(pipeline) = object.as_pipeline() {
            let mut ctx = self.clone();
            for item in &pipeline.items {
                ctx = ctx.alter_value(item.call(item.arguments.clone(), ctx.clone()).await.err_prefix(err_prefix.as_ref())?);
            }
            Ok(ctx.value().clone())
        } else {
            Ok(object)
        }
    }

    fn alter_value(&self, value: Object) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                value,
                object: self.inner.object.clone(),
                path: self.inner.path.clone(),
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