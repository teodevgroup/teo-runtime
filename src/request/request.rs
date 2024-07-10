use std::cell::Ref;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use actix_http::header::HeaderMap;
use actix_http::HttpMessage;
use actix_web::cookie::Cookie;
use actix_web::HttpRequest;
use teo_result::{Error, Result};
use crate::request::Ctx;
use crate::request::ctx::extract::ExtractFromRequestCtx;

#[derive(Clone)]
pub struct Request {
    inner: Arc<HttpRequest>
}

impl Request {

    pub fn new(actix_http_request: HttpRequest) -> Self {
        Self { inner: Arc::new(actix_http_request) }
    }

    pub fn method(&self) -> &str {
        self.inner.method().as_str()
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

    pub fn cookies(&self) -> Result<Ref<Vec<Cookie>>> {
        match self.inner.cookies() {
            Ok(cookies) => Ok(cookies),
            Err(_) => Err(Error::invalid_request_message("invalid cookie format")),
        }
    }
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("method", &self.inner.method());
        debug_struct.field("path", &self.inner.path());
        debug_struct.field("query_string", &self.inner.query_string());
        debug_struct.field("content_type", &self.inner.content_type());
        debug_struct.field("headers", &self.inner.headers());
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