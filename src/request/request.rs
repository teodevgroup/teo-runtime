use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use hyper::body::Incoming;
use hyper::{HeaderMap, Uri};
use hyper::http::HeaderValue;
use crate::request::Ctx;
use crate::request::ctx::extract::ExtractFromRequestCtx;

#[derive(Clone)]
pub struct Request {
    inner: Arc<hyper::Request<Incoming>>
}

impl Request {

    pub fn new(inner: Arc<hyper::Request<Incoming>>) -> Self {
        Self { inner }
    }

    pub fn method(&self) -> &str {
        self.inner.method().as_str()
    }

    pub fn uri(&self) -> &Uri {
        self.inner.uri()
    }

    pub fn path(&self) -> &str {
        self.inner.uri().path()
    }

    pub fn query_string(&self) -> &str {
        self.inner.uri().query().unwrap_or("")
    }

    pub fn content_type(&self) -> &str {
        self.inner.headers().get("content-type").map(|c| c.to_str().unwrap()).unwrap_or("")
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.inner.headers()
    }

    // pub fn cookies(&self) -> Result<Vec<Cookie>> {
    //     self.inner.cookies()
    // }
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("method", &self.method());
        debug_struct.field("path", &self.path());
        debug_struct.field("query_string", &self.query_string());
        debug_struct.field("content_type", &self.content_type());
        debug_struct.finish()
    }
}

impl ExtractFromRequestCtx for Request {
    fn extract(ctx: &Ctx) -> Self {
        ctx.request().clone()
    }
}

unsafe impl Send for Request { }
unsafe impl Sync for Request { }