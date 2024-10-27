use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use hyper::{self, header::{HeaderMap, HeaderValue}, Uri, Version};
use teo_result::{Error, Result};
use crate::request::{Cookie, Ctx};
use crate::request::cookies::Cookies;
use crate::request::ctx::extract::ExtractFromRequestCtx;

#[derive(Clone)]
pub struct Request {
    inner: Arc<hyper::Request<hyper::body::Incoming>>,
    cookies: Arc<Mutex<Option<Cookies>>>,
}

impl Request {

    pub fn version(&self) -> Version {
        self.inner.version()
    }

    pub fn method(&self) -> &str {
        self.inner.method().as_str()
    }

    pub fn uri(&self) -> &Uri {
        self.inner.uri()
    }

    pub fn uri_string(&self) -> String {
        self.inner.uri().to_string()
    }

    pub fn path(&self) -> &str {
        self.inner.uri().path()
    }

    pub fn query(&self) -> Option<&str> {
        self.inner.uri().query()
    }

    pub fn host(&self) -> Option<&str> {
        self.inner.uri().host()
    }

    pub fn content_type(&self) -> Result<Option<&str>> {
        if let Some(value) = self.inner.headers().get("content-type") {
            match value.to_str() {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(Error::internal_server_error_message("cannot read request header value: content-type")),
            }
        } else {
            Ok(None)
        }
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.inner.headers()
    }

    pub fn contains_header(&self, header_key: &str) -> bool {
        self.inner.headers().contains_key(header_key)
    }

    pub fn header(&self, header_key: &str) -> Result<Option<&str>> {
        match self.inner.headers().get(header_key) {
            Some(value) => match value.to_str() {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(Error::internal_server_error_message(format!("cannot read request header value: {}", header_key))),
            },
            None => Ok(None),
        }
    }

    pub fn cookies(&self) -> Result<Cookies> {
        if self.cookies.lock().unwrap().is_none() {
            self.parse_cookies()
        } else {
            Ok(self.cookies.lock().unwrap().as_ref().unwrap().clone())
        }
    }

    fn parse_cookies(&self) -> Result<Cookies> {
        let mut cookies = Vec::new();
        for cookie_header_value in self.inner.headers().get_all("cookie") {
            let cookie_full_str = cookie_header_value.to_str().map_err(|_| Error::internal_server_error_message("cannot read request header value: cookie"))?;
            for cookie_str in cookie_full_str.split(';').map(|s| s.trim()) {
                if !cookie_str.is_empty() {
                    cookies.push(match Cookie::parse_encoded(cookie_str) {
                        Ok(cookie) => cookie,
                        Err(_) => return Err(Error::internal_server_error_message(format!("invalid cookie format: `{}`", cookie_str))),
                    }.into_owned());
                }
            }
        }
        let cookies = Cookies::from(cookies);
        self.cookies.lock().unwrap().replace(cookies.clone());
        Ok(cookies)
    }
}

impl From<hyper::Request<hyper::body::Incoming>> for Request {
    fn from(hyper_request: hyper::Request<hyper::body::Incoming>) -> Self {
        Self {
            inner: Arc::new(hyper_request),
            cookies: Arc::new(Mutex::new(None)),
        }
    }
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("version", &self.inner.version());
        debug_struct.field("method", &self.inner.method());
        debug_struct.field("uri", &self.inner.uri());
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