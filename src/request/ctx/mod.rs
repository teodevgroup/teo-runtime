use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;
use teo_teon::Value;
use crate::request::Request;
use crate::connection::transaction;
use crate::handler::r#match::HandlerMatch;
use crate::namespace::Namespace;
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
    body: Arc<Value>,
    transaction_ctx: transaction::Ctx,
    handler_match: HandlerMatch,
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

    pub fn transaction_ctx(&self) -> &transaction::Ctx {
        &self.inner.transaction_ctx
    }

    pub fn namespace(&self) -> &'static Namespace {
        self.inner.transaction_ctx.namespace()
    }

    pub fn handler_match(&self) -> &HandlerMatch {
        &self.inner.handler_match
    }

    pub fn body(&self) -> &Value {
        self.inner.body.as_ref()
    }
}

