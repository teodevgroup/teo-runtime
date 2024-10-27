pub mod extract;

use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;
use crate::value::Value;
use crate::request::Request;
use crate::connection::transaction;
use crate::handler::r#match::HandlerMatch;
use crate::namespace::Namespace;
use crate::request::ctx::extract::ExtractFromRequestCtx;
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
    data: RefCell<Data>,
}

impl Ctx {

    pub fn new(request: Request, body: Arc<Value>, transaction_ctx: transaction::Ctx) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                request,
                body,
                transaction_ctx,
                data: RefCell::new(Data::new())
            })
        }
    }

    pub fn request(&self) -> &Request {
        &self.inner.request
    }

    pub fn data(&self) -> Ref<Data> {
        self.inner.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<Data> {
        self.inner.data.borrow_mut()
    }

    pub fn transaction_ctx(&self) -> transaction::Ctx {
        self.inner.transaction_ctx.clone()
    }

    pub fn namespace(&self) -> &'static Namespace {
        self.inner.transaction_ctx.namespace()
    }

    pub fn body(&self) -> &Value {
        self.inner.body.as_ref()
    }
}

impl ExtractFromRequestCtx for Ctx {
    fn extract(ctx: &Ctx) -> Self {
        ctx.clone()
    }
}
