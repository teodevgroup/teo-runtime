use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use hyper::{self, header::{HeaderMap, HeaderValue}, Method, Uri, Version};
use teo_result::{Error, Result};
use cookie::Cookie;
use deferred_box::DeferredBox;
use history_box::HistoryBox;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::header::CONTENT_TYPE;
use hyper::http::uri::Scheme;
use indexmap::IndexMap;
use bytes::Bytes;
use crate::connection::transaction;
use crate::handler::r#match::HandlerMatch;
use crate::request::cookies::Cookies;
use crate::request::extract::ExtractFromRequest;
use crate::request::local_objects::LocalObjects;
use crate::request::local_values::LocalValues;
use crate::Value;

#[derive(Clone)]
pub struct Request {
    inner: Arc<Inner>,
}

struct Inner {
    hyper_request: hyper::Request<()>,
    transaction_ctx: transaction::Ctx,
    cookies: DeferredBox<Cookies>,
    handler_match: HistoryBox<HandlerMatch>,
    body_value: HistoryBox<Value>,
    local_values: RefCell<LocalValues>,
    local_objects: RefCell<LocalObjects>,
    incoming: RefCell<Option<Incoming>>,
    incoming_bytes: RefCell<Option<Full<Bytes>>>,
}

impl Request {

    pub fn new(hyper_request: hyper::Request<Incoming>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        let hyper_request = hyper::Request::from_parts(parts, ());
        Self {
            inner: Arc::new(Inner {
                hyper_request,
                incoming: RefCell::new(Some(incoming)),
                incoming_bytes: RefCell::new(None),
                transaction_ctx,
                cookies: DeferredBox::new(),
                handler_match: HistoryBox::new(),
                body_value: HistoryBox::new(),
                local_values: RefCell::new(LocalValues::new()),
                local_objects: RefCell::new(LocalObjects::new()),
            })
        }
    }

    pub fn new_for_test(hyper_request: hyper::Request<Full<Bytes>>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        let hyper_request = hyper::Request::from_parts(parts, ());
        Self {
            inner: Arc::new(Inner {
                hyper_request,
                incoming: RefCell::new(None),
                incoming_bytes: RefCell::new(Some(incoming)),
                transaction_ctx,
                cookies: DeferredBox::new(),
                handler_match: HistoryBox::new(),
                body_value: HistoryBox::new(),
                local_values: RefCell::new(LocalValues::new()),
                local_objects: RefCell::new(LocalObjects::new()),
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

    pub fn header_value(&self, header_key: &str) -> Result<Option<&str>> {
        match self.inner.hyper_request.headers().get(header_key) {
            Some(value) => match value.to_str() {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(Error::internal_server_error_message(format!("cannot read request header value: {}", header_key))),
            },
            None => Ok(None),
        }
    }

    pub fn cookies(&self) -> Result<&Cookies> {
        if self.inner.cookies.get().is_none() {
            self.parse_cookies()
        } else {
            Ok(self.inner.cookies.get().unwrap())
        }
    }

    pub fn handler_match(&self) -> Result<&HandlerMatch> {
        match self.inner.handler_match.get() {
            Some(handler_match) => Ok(handler_match),
            None => Err(Error::internal_server_error_message("handler match is accessed while it is unavailable")),
        }
    }

    pub fn set_handler_match(&self, handler_match: HandlerMatch) {
        self.inner.handler_match.set(handler_match);
    }

    pub fn captures(&self) -> Result<&IndexMap<String, String>> {
        Ok(self.handler_match()?.captures())
    }

    pub fn body_value(&self) -> Result<&Value> {
        match self.inner.body_value.get() {
            Some(value) => Ok(value),
            None => Err(Error::internal_server_error_message("request body value is accessed while it is unavailable")),
        }
    }

    pub fn set_body_value(&self, value: Value) {
        self.inner.body_value.set(value)
    }

    pub fn transaction_ctx(&self) -> transaction::Ctx {
        self.inner.transaction_ctx.clone()
    }

    pub fn local_values(&self) -> Ref<LocalValues> {
        self.inner.local_values.borrow()
    }

    pub fn local_objects(&self) -> Ref<LocalObjects> {
        self.inner.local_objects.borrow()
    }

    pub fn take_incoming(&self) -> Option<Incoming> {
        self.inner.incoming.replace(None)
    }

    pub fn take_incoming_bytes_for_test(&self) -> Option<Full<Bytes>> {
        self.inner.incoming_bytes.replace(None)
    }

    pub fn clone_hyper_request(&self) -> hyper::Request<()> {
        self.inner.hyper_request.clone()
    }

    fn parse_cookies(&self) -> Result<&Cookies> {
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
        self.inner.cookies.set(cookies).unwrap();
        Ok(self.inner.cookies.get().unwrap())
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

impl<'a> ExtractFromRequest<'a> for Request {
    fn extract(request: &'a Request) -> Self {
        request.clone()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a Request {
    fn extract(request: &'a Request) -> Self {
        request
    }
}

unsafe impl Send for Request { }
unsafe impl Sync for Request { }
