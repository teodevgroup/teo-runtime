use std::cell::{Ref, RefCell, RefMut};
use std::collections::BTreeMap;
use std::sync::Arc;
use teo_teon::Value;
use crate::request::Request;
use crate::connection;
use super::local::Data;

#[derive(Debug, Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>
}

#[derive(Debug)]
struct CtxInner {
    request: Request,
    body: Value,
    connection_ctx: connection::Ctx,
    // pub(crate) path_components: PathComponents,
    //pub action: Option<Action>,
    data: RefCell<Data>,
}

impl Ctx {

    pub fn request(&self) -> &Request {
        &self.inner.request
    }

    pub fn data(&self) -> Ref<Data> {
        self.inner.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<Data> {
        self.inner.data.borrow_mut()
    }
}

unsafe impl Send for Ctx {}
