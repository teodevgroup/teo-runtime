use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use hyper::{self, header::{HeaderMap, HeaderValue}, Method, Uri, Version};
use teo_result::{Error, Result};
use cookie::Cookie;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use crate::connection::transaction;
use crate::handler::r#match::HandlerMatch;
use crate::request::cookies::Cookies;
use crate::request::extract::ExtractFromRequest;
use crate::request::local::RequestLocal;
use crate::Value;

#[derive(Clone)]
pub struct Request {
    inner: Arc<hyper::Request<()>>,
    transaction_ctx: transaction::Ctx,
    cookies: Arc<Mutex<Option<Cookies>>>,
    handler_match: Arc<Mutex<Option<HandlerMatch>>>,
    body_value: Arc<Mutex<Arc<Value>>>,
    local_data: Arc<RefCell<RequestLocal>>,
    local_objects: Arc<RefCell<RequestLocal>>,
    incoming: Arc<RefCell<Option<Incoming>>>,
}

impl Request {

    pub fn new(hyper_request: hyper::Request<Incoming>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        let hyper_request = hyper::Request::from_parts(parts, ());
        Self {
            inner: Arc::new(hyper_request),
            incoming: Arc::new(RefCell::new(Some(incoming))),
            transaction_ctx,
            cookies: Arc::new(Mutex::new(None)),
            handler_match: Arc::new(Mutex::new(None)),
            body_value: Arc::new(Mutex::new(Arc::new(Value::Null))),
            local_data: Arc::new(RefCell::new(RequestLocal::new())),
            local_objects: Arc::new(RefCell::new(RequestLocal::new())),
        }
    }

    pub fn version(&self) -> Version {
        self.inner.version()
    }

    pub fn method(&self) -> &Method {
        self.inner.method()
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

    pub fn handler_match(&self) -> Option<HandlerMatch> {
        self.handler_match.lock().unwrap().clone()
    }

    pub fn set_handler_match(&self, handler_match: HandlerMatch) {
        self.handler_match.lock().unwrap().replace(handler_match);
    }

    pub fn body_value(&self) -> Arc<Value> {
        self.body_value.lock().unwrap().clone()
    }

    pub fn set_body_value(&self, value: Value) {
        *self.body_value.lock().unwrap() = Arc::new(value);
    }

    pub fn transaction_ctx(&self) -> transaction::Ctx {
        self.transaction_ctx.clone()
    }

    pub fn local_data(&self) -> Ref<RequestLocal> {
        self.local_data.borrow()
    }

    pub fn local_data_mut(&self) -> RefMut<RequestLocal> {
        self.local_data.borrow_mut()
    }

    pub fn local_objects(&self) -> Ref<RequestLocal> {
        self.local_objects.borrow()
    }

    pub fn local_objects_mut(&self) -> RefMut<RequestLocal> {
        self.local_objects.borrow_mut()
    }

    pub fn take_incoming(&self) -> Option<Incoming> {
        self.incoming.replace(None)
    }

    fn parse_cookies(&self) -> Result<Cookies> {
        let mut cookies: Vec<Cookie<'static>> = Vec::new();
        for cookie_header_value in self.inner.headers().get_all("cookie") {
            let cookie_full_str = cookie_header_value.to_str().map_err(|_| Error::internal_server_error_message("cannot read request header value: cookie"))?;
            for cookie_str in cookie_full_str.split(';').map(|s| s.trim()) {
                if !cookie_str.is_empty() {
                    cookies.push(match Cookie::parse_encoded(cookie_str) {
                        Ok(cookie) => cookie,
                        Err(_) => return Err(Error::invalid_request_message(format!("invalid cookie format: `{}`", cookie_str))),
                    }.into_owned());
                }
            }
        }
        let cookies = Cookies::from(cookies);
        self.cookies.lock().unwrap().replace(cookies.clone());
        Ok(cookies)
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

impl ExtractFromRequest for Request {
    fn extract(request: &Request) -> Self {
        request.clone()
    }
}

unsafe impl Send for Request { }
unsafe impl Sync for Request { }
