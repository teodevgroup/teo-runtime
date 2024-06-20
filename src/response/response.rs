use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::value::Value;
use crate::teon;
use teo_result::Result;
use crate::response::body::Body;
use crate::response::header::readwrite::HeaderMap;

#[derive(Debug)]
pub struct Response {
    inner: hyper::Response<Vec<u8>>,
}

impl Response {

    pub fn empty() -> Response {
        Self {
            inner: hyper::Response::builder().body(vec![]).unwrap(),
        }
    }

    pub fn string(content: impl Into<String>, content_type: &str) -> Response {
        let string: String = content.into();
        let inner = hyper::Response::builder()
            .header("content-type", content_type)
            .body(string.into_bytes()).unwrap();
        Self { inner }
    }

    pub fn teon(value: Value) -> Response {
        let inner = hyper::Response::builder()
            .header("content-type", "application/json")
            .body(serde_json::Value::from(&value).to_string().into_bytes()).unwrap();
        Self { inner }
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
}

pub struct ResponseInner {
    code: u16,
    headers: HeaderMap,
    body: Body,
}

impl ResponseInner {

    fn new() -> Self {
        Self {
            code: 200,
            headers: HeaderMap::new(),
            body: Body::empty(),
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
