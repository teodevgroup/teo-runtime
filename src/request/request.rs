use std::cell::{Ref, RefCell, RefMut, UnsafeCell};
use std::fmt::{Debug, Formatter};
use std::ptr::null;
use std::sync::{Arc, Mutex};
use hyper::{self, header::{HeaderMap, HeaderValue}, Method, Uri, Version};
use teo_result::{Error, Result};
use cookie::Cookie;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::header::CONTENT_TYPE;
use hyper::http::uri::Scheme;
use crate::connection::transaction;
use crate::handler::r#match::HandlerMatch;
use crate::request::cookies::Cookies;
use crate::request::extract::ExtractFromRequest;
use crate::request::local::RequestLocal;
use crate::Value;

#[derive(Clone)]
pub struct Request {
    inner: Arc<Inner>,
}

struct Inner {
    hyper_request: hyper::Request<()>,
    transaction_ctx: transaction::Ctx,
    cookies: Arc<Mutex<Option<Cookies>>>,
    handler_match: Arc<Mutex<Option<HandlerMatch>>>,
    body_value: Arc<Mutex<* const Value>>,
    body_values: UnsafeCell<Vec<Box<Value>>>,
    local_data: RefCell<RequestLocal>,
    local_objects: RefCell<RequestLocal>,
    incoming: RefCell<Option<Incoming>>,
    incoming_string: RefCell<Option<String>>,
}

impl Request {

    pub fn new(hyper_request: hyper::Request<Incoming>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        let hyper_request = hyper::Request::from_parts(parts, ());
        Self {
            inner: Arc::new(Inner {
                hyper_request,
                incoming: RefCell::new(Some(incoming)),
                incoming_string: RefCell::new(None),
                transaction_ctx,
                cookies: Arc::new(Mutex::new(None)),
                handler_match: Arc::new(Mutex::new(None)),
                body_value: Arc::new(Mutex::new(null())),
                body_values: UnsafeCell::new(Vec::new()),
                local_data: RefCell::new(RequestLocal::new()),
                local_objects: RefCell::new(RequestLocal::new()),
            })
        }
    }

    pub fn new_for_test(hyper_request: hyper::Request<String>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        let hyper_request = hyper::Request::from_parts(parts, ());
        Self {
            inner: Arc::new(Inner {
                hyper_request,
                incoming: RefCell::new(None),
                incoming_string: RefCell::new(Some(incoming)),
                transaction_ctx,
                cookies: Arc::new(Mutex::new(None)),
                handler_match: Arc::new(Mutex::new(None)),
                body_value: Arc::new(Mutex::new(null())),
                body_values: UnsafeCell::new(Vec::new()),
                local_data: RefCell::new(RequestLocal::new()),
                local_objects: RefCell::new(RequestLocal::new()),
            })
        }
    }

    pub fn version(&self) -> Version {
        self.inner.hyper_request.version()
    }

    pub fn method(&self) -> &Method {
        self.inner.hyper_request.method()
    }

    pub fn uri(&self) -> &Uri {
        self.inner.hyper_request.uri()
    }

    pub fn uri_string(&self) -> String {
        self.inner.hyper_request.uri().to_string()
    }

    pub fn scheme(&self) -> Option<&Scheme> {
        self.inner.hyper_request.uri().scheme()
    }

    pub fn scheme_str(&self) -> Option<&str> {
        self.inner.hyper_request.uri().scheme_str()
    }

    pub fn host(&self) -> Option<&str> {
        self.inner.hyper_request.uri().host()
    }

    pub fn path(&self) -> &str {
        self.inner.hyper_request.uri().path()
    }

    pub fn query(&self) -> Option<&str> {
        self.inner.hyper_request.uri().query()
    }

    pub fn content_type(&self) -> Result<Option<&str>> {
        if let Some(value) = self.inner.hyper_request.headers().get(CONTENT_TYPE.as_str()) {
            match value.to_str() {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(Error::internal_server_error_message("cannot read request header value: content-type")),
            }
        } else {
            Ok(None)
        }
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.inner.hyper_request.headers()
    }

    pub fn contains_header(&self, header_key: &str) -> bool {
        self.inner.hyper_request.headers().contains_key(header_key)
    }

    pub fn header(&self, header_key: &str) -> Result<Option<&str>> {
        match self.inner.hyper_request.headers().get(header_key) {
            Some(value) => match value.to_str() {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(Error::internal_server_error_message(format!("cannot read request header value: {}", header_key))),
            },
            None => Ok(None),
        }
    }

    pub fn cookies(&self) -> Result<Cookies> {
        if self.inner.cookies.lock().unwrap().is_none() {
            self.parse_cookies()
        } else {
            Ok(self.inner.cookies.lock().unwrap().as_ref().unwrap().clone())
        }
    }

    pub fn handler_match(&self) -> Option<HandlerMatch> {
        self.inner.handler_match.lock().unwrap().clone()
    }

    pub fn set_handler_match(&self, handler_match: HandlerMatch) {
        self.inner.handler_match.lock().unwrap().replace(handler_match);
    }

    pub fn body_value(&self) -> Result<&Value> {
        let pointer = *match self.inner.body_value.lock() {
            Ok(pointer) => pointer,
            Err(err) => return Err(Error::internal_server_error_message(format!("cannot lock request body value: {}", err))),
        };
        if pointer.is_null() {
            return Err(Error::internal_server_error_message("request body value is accessed while it is unavailable"));
        }
        Ok(unsafe { &*pointer })
    }

    pub fn set_body_value(&self, value: Value) {
        let body_values = unsafe { &mut *self.inner.body_values.get() };
        body_values.push(Box::new(value));
        let pointer = body_values.last().unwrap().as_ref();
        *self.inner.body_value.lock().unwrap() = pointer;
    }

    pub fn transaction_ctx(&self) -> transaction::Ctx {
        self.inner.transaction_ctx.clone()
    }

    pub fn local_data(&self) -> Ref<RequestLocal> {
        self.inner.local_data.borrow()
    }

    pub fn local_data_mut(&self) -> RefMut<RequestLocal> {
        self.inner.local_data.borrow_mut()
    }

    pub fn local_objects(&self) -> Ref<RequestLocal> {
        self.inner.local_objects.borrow()
    }

    pub fn local_objects_mut(&self) -> RefMut<RequestLocal> {
        self.inner.local_objects.borrow_mut()
    }

    pub fn take_incoming(&self) -> Option<Incoming> {
        self.inner.incoming.replace(None)
    }

    pub fn take_incoming_string_for_test(&self) -> Option<String> {
        self.inner.incoming_string.replace(None)
    }

    pub fn clone_hyper_request(&self) -> hyper::Request<()> {
        self.inner.hyper_request.clone()
    }

    fn parse_cookies(&self) -> Result<Cookies> {
        let mut cookies: Vec<Cookie<'static>> = Vec::new();
        for cookie_header_value in self.inner.hyper_request.headers().get_all("cookie") {
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
        self.inner.cookies.lock().unwrap().replace(cookies.clone());
        Ok(cookies)
    }
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("version", &self.inner.hyper_request.version());
        debug_struct.field("method", &self.inner.hyper_request.method());
        debug_struct.field("uri", &self.inner.hyper_request.uri());
        debug_struct.field("headers", &self.inner.hyper_request.headers());
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
