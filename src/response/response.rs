use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use teo_teon::{teon, Value};
use teo_result::{Result};
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

    pub fn json(value: Value) -> Result<Response> {
        let mut inner = ResponseInner::new();
        let json_value = serde_json::Value::try_from(value)?;
        let string_value = serde_json::to_string(&json_value).unwrap();
        inner.body = Body::string(string_value);
        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn data(value: Value) -> Result<Response> {
        Self::json(teon!({"data": value}))
    }

    pub fn data_meta(data: Value, meta: Value) -> Result<Response> {
        Self::json(teon!({"data": data, "meta": meta}))
    }

    pub fn error(error: impl Into<crate::path::Error>) -> Result<Response> {
        let path_error = error.into();
        let code = path_error.code;
        let value: Value = path_error.into();
        let res = Self::json(value)?;
        res.set_code(code as u16);
        Ok(res)
    }

    pub fn file(path: PathBuf) -> Response {
        let mut res = Self::empty();
        res.inner.lock().unwrap().body = Body::file(path);
        res
    }

    pub fn redirect(path: impl Into<String>) -> Response {
        let mut res = Self::empty();
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
