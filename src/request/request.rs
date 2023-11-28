use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::request::Ctx;
use crate::request::ctx::extract::ExtractFromRequestCtx;
use crate::request::header::readonly::HeaderMap;

#[derive(Clone)]
pub struct Request {
    inner: Arc<dyn r#trait::Request>
}

impl Request {

    pub fn new(inner: Arc<dyn r#trait::Request>) -> Self {
        Self { inner }
    }

    pub fn method(&self) -> &str {
        self.inner.method()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }

    pub fn query_string(&self) -> &str {
        self.inner.query_string()
    }

    pub fn content_type(&self) -> &str {
        self.inner.content_type()
    }

    pub fn headers(&self) -> &HeaderMap {
        self.inner.headers()
    }
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("method", &self.inner.method());
        debug_struct.field("path", &self.inner.path());
        debug_struct.field("query_string", &self.inner.query_string());
        debug_struct.field("content_type", &self.inner.content_type());
        debug_struct.finish()
    }
}

pub mod r#trait {
    use crate::request::header::readonly::HeaderMap;

    pub trait Request {
        fn method(&self) -> &str;
        fn path(&self) -> &str;
        fn query_string(&self) -> &str;
        fn content_type(&self) -> &str;
        fn headers(&self) -> &HeaderMap;
    }
}

impl ExtractFromRequestCtx for Request {
    fn extract(ctx: &Ctx) -> Self {
        ctx.request().clone()
    }
}