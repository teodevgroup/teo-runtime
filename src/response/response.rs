use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use cookie::Cookie;
use hyper::header::CONTENT_TYPE;
use crate::value::Value;
use crate::teon;
use teo_result::{Result, Error};
use crate::response::body::Body;
use crate::response::header::readwrite::HeaderMap;

#[derive(Clone)]
pub struct Response {
    inner: Arc<Mutex<ResponseInner>>
}

impl Response {

    pub fn empty() -> Response {
        Self {
            inner: Arc::new(Mutex::new(ResponseInner::new())),
        }
    }

    pub fn string(content: impl Into<String>, content_type: &str) -> Response {
        let mut inner = ResponseInner::new();
        inner.body = Body::string(content.into());
        inner.headers.set(CONTENT_TYPE.as_str(), content_type);
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn teon(value: Value) -> Response {
        let mut inner = ResponseInner::new();
        inner.body = Body::teon(value);
        Self {
            inner: Arc::new(Mutex::new(inner))
        }
    }

    pub fn text(content: impl Into<String>) -> Result<Response> {
        Ok(Self::string(content.into(), "text/plain"))
    }

    pub fn html(content: impl Into<String>) -> Result<Response> {
        Ok(Self::string(content.into(), "text/html"))
    }

    pub fn data(value: Value) -> Response {
        Self::teon(teon!({"data": value}))
    }

    pub fn data_meta(data: Value, meta: Value) -> Response {
        Self::teon(teon!({"data": data, "meta": meta}))
    }

    pub fn file(path: PathBuf) -> Response {
        let res = Self::empty();
        res.inner.lock().unwrap().body = Body::file(path);
        res
    }

    pub fn send_file(base: impl AsRef<str>, path: impl AsRef<str>) -> Result<Response> {
        let base_str = base.as_ref();
        let path_str = path.as_ref();
        let combined_path = PathBuf::from(base_str).join(path_str);
        if combined_path.is_file() {
            Ok(Response::file(combined_path))
        } else {
            Err(Error::not_found())
        }
    }

    pub fn redirect(path: impl Into<String>) -> Response {
        let res = Self::empty();
        res.set_code(301);
        res.headers().set("location", path.into());
        res
    }

    pub fn set_code(&self, code: u16) {
        self.inner.lock().unwrap().code = code;
    }

    pub fn code(&self) -> u16 {
        self.inner.lock().unwrap().code
    }

    pub fn headers(&self) -> HeaderMap {
        self.inner.lock().unwrap().headers.clone()
    }

    pub fn body(&self) -> Body {
        self.inner.lock().unwrap().body.clone()
    }

    pub fn add_cookie(&self, cookie: Cookie<'static>) {
        self.inner.lock().unwrap().cookies.push(cookie);
    }

    pub fn cookies(&self) -> Vec<Cookie<'static>> {
        self.inner.lock().unwrap().cookies.clone()
    }
}

pub struct ResponseInner {
    code: u16,
    headers: HeaderMap,
    body: Body,
    cookies: Vec<Cookie<'static>>,
}

impl ResponseInner {

    fn new() -> Self {
        Self {
            code: 200,
            headers: HeaderMap::new(),
            body: Body::empty(),
            cookies: vec![],
        }
    }
}

impl Debug for Response {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Response");
        debug_struct.field("code", &self.inner.lock().unwrap().code);
        debug_struct.field("headers", &self.inner.lock().unwrap().headers);
        debug_struct.finish()
    }
}
