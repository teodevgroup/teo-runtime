use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;
use teo_teon::Value;
use crate::request::Request;
use crate::connection::transaction;
use super::local::Data;

#[derive(Debug, Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>
}

unsafe impl Send for Ctx { }
unsafe impl Sync for Ctx { }

#[derive(Debug)]
struct CtxInner {
    request: Request,
    body: Value,
    transaction_ctx: transaction::Ctx,
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

