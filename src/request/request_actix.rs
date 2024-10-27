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
pub struct RequestActix {
    inner: Arc<HttpRequest>
}

impl RequestActix {

    pub fn new(actix_http_request: HttpRequest) -> Self {
        Self { inner: Arc::new(actix_http_request) }
    }


}
