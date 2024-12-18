use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use hyper::{self, Method, Uri, Version};
use teo_result::{Error, Result};
use history_box::HistoryBox;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::header::CONTENT_TYPE;
use hyper::http::uri::Scheme;
use indexmap::IndexMap;
use bytes::Bytes;
use crate::connection::transaction;
use crate::handler::r#match::HandlerMatch;
use crate::cookies::Cookies;
use crate::headers::headers::Headers;
use crate::request::extract::ExtractFromRequest;
use crate::request::local_objects::LocalObjects;
use crate::request::local_values::LocalValues;
use crate::Value;

#[derive(Clone)]
pub struct Request {
    inner: Arc<Inner>,
}

struct Inner {
    method: HistoryBox<Method>,
    uri: HistoryBox<Uri>,
    version: HistoryBox<Version>,
    headers: HistoryBox<Headers>,
    transaction_ctx: transaction::Ctx,
    cookies: HistoryBox<Cookies>,
    handler_match: HistoryBox<HandlerMatch>,
    body_value: HistoryBox<Value>,
    local_values: LocalValues,
    local_objects: LocalObjects,
    incoming: RefCell<Option<Incoming>>,
    incoming_bytes: RefCell<Option<Full<Bytes>>>,
}

impl Request {

    pub fn new(hyper_request: hyper::Request<Incoming>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        Self {
            inner: Arc::new(Inner {
                method: HistoryBox::new_with(parts.method),
                uri: HistoryBox::new_with(parts.uri),
                version: HistoryBox::new_with(parts.version),
                headers: HistoryBox::new_with(Headers::from(parts.headers)),
                incoming: RefCell::new(Some(incoming)),
                incoming_bytes: RefCell::new(None),
                transaction_ctx,
                cookies: HistoryBox::new(),
                handler_match: HistoryBox::new(),
                body_value: HistoryBox::new(),
                local_values: LocalValues::new(),
                local_objects: LocalObjects::new(),
            })
        }
    }

    pub fn new_for_test(hyper_request: hyper::Request<Full<Bytes>>, transaction_ctx: transaction::Ctx) -> Self {
        let (parts, incoming) = hyper_request.into_parts();
        Self {
            inner: Arc::new(Inner {
                method: HistoryBox::new_with(parts.method),
                uri: HistoryBox::new_with(parts.uri),
                version: HistoryBox::new_with(parts.version),
                headers: HistoryBox::new_with(Headers::from(parts.headers)),
                incoming: RefCell::new(None),
                incoming_bytes: RefCell::new(Some(incoming)),
                transaction_ctx,
                cookies: HistoryBox::new(),
                handler_match: HistoryBox::new(),
                body_value: HistoryBox::new(),
                local_values: LocalValues::new(),
                local_objects: LocalObjects::new(),
            })
        }
    }

    #[inline(always)]
    pub fn version(&self) -> Version {
        *self.inner.version.get().unwrap()
    }

    #[inline(always)]
    pub fn set_version(&self, version: Version) {
        self.inner.version.set(version);
    }

    #[inline(always)]
    pub fn method(&self) -> &Method {
        self.inner.method.get().unwrap()
    }

    #[inline(always)]
    pub fn set_method(&self, method: Method) {
        self.inner.method.set(method);
    }

    #[inline(always)]
    pub fn uri(&self) -> &Uri {
        self.inner.uri.get().unwrap()
    }

    #[inline(always)]
    pub fn set_uri(&self, uri: Uri) {
        self.inner.uri.set(uri);
    }

    pub fn set_uri_string(&self, uri: &str) -> Result<()> {
        self.inner.uri.set(Uri::from_str(uri)?);
        Ok(())
    }

    #[inline(always)]
    pub fn uri_string(&self) -> String {
        self.uri().to_string()
    }

    #[inline(always)]
    pub fn scheme(&self) -> Option<&Scheme> {
        self.uri().scheme()
    }

    #[inline(always)]
    pub fn scheme_str(&self) -> Option<&str> {
        self.uri().scheme_str()
    }

    #[inline(always)]
    pub fn host(&self) -> Option<&str> {
        self.uri().host()
    }

    #[inline(always)]
    pub fn path(&self) -> &str {
        self.uri().path()
    }

    #[inline(always)]
    pub fn query(&self) -> Option<&str> {
        self.uri().query()
    }

    pub fn content_type(&self) -> Result<Option<String>> {
        if let Some(value) = self.headers().get(CONTENT_TYPE.as_str())? {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn headers(&self) -> Headers {
        self.inner.headers.get().unwrap().clone()
    }

    pub fn set_headers(&self, headers: Headers) {
        self.inner.headers.set(headers);
    }

    pub fn cookies(&self) -> Result<&Cookies> {
        if self.inner.cookies.get().is_none() {
            self.parse_cookies()
        } else {
            Ok(self.inner.cookies.get().unwrap())
        }
    }

    pub fn set_cookies(&self, cookies: Cookies) {
        self.inner.cookies.set(cookies);
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

    pub fn local_values(&self) -> &LocalValues {
        &self.inner.local_values
    }

    pub fn local_objects(&self) -> &LocalObjects {
        &self.inner.local_objects
    }

    pub fn take_incoming(&self) -> Option<Incoming> {
        self.inner.incoming.replace(None)
    }

    pub fn take_incoming_bytes_for_test(&self) -> Option<Full<Bytes>> {
        self.inner.incoming_bytes.replace(None)
    }

    pub fn clone_hyper_request_for_file_processing(&self) -> hyper::Request<()> {
        let mut request = hyper::Request::builder()
            .method(self.inner.method.get().unwrap().clone())
            .uri(self.inner.uri.get().unwrap().clone())
            .version(self.inner.version.get().unwrap().clone())
            .body(())
            .unwrap();
        self.headers().extend_to(request.headers_mut());
        request
    }

    fn parse_cookies(&self) -> Result<&Cookies> {
        self.inner.cookies.set(Cookies::from_request_headers(&self.headers())?);
        Ok(self.inner.cookies.get().unwrap())
    }
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("version", self.inner.version.get().unwrap());
        debug_struct.field("method", &self.inner.method.get().unwrap());
        debug_struct.field("uri", &self.inner.uri.get().unwrap());
        debug_struct.field("headers", &self.inner.headers);
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
