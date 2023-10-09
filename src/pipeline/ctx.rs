use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use key_path::KeyPath;
use crate::object::Object;
use crate::request::Request;
use crate::model;

#[derive(Clone)]
pub struct Ctx<'a> {
    inner: Arc<CtxInner<'a>>,
}

#[derive(Debug)]
struct CtxInner<'a> {
    value: Object,
    object: model::Object,
    path: KeyPath<'a>,
    //pub(crate) action: Action,
    //pub(crate) conn: Arc<dyn Connection>,
    request: Option<Request>,
}

impl<'a> Ctx<'a> {

    pub fn value(&self) -> &Object {
        &self.inner.value
    }

    pub fn object(&self) -> &model::Object {
        &self.inner.object
    }

    pub fn path(&self) -> &KeyPath<'a> {
        &self.inner.path
    }

    pub fn request(&self) -> Option<&Request> {
        self.inner.request.as_ref()
    }
}

impl<'a> Debug for Ctx<'a> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.inner.as_ref(), f)
    }
}