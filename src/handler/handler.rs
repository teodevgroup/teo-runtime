use std::future::Future;
use educe::Educe;
use futures_util::future::BoxFuture;
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::request::ctx::Ctx;
use crate::response::Response;
use teo_result::Result;
use crate::middleware::next::Next;

#[derive(Debug, Serialize, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Options,
}

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize, Clone)]
pub struct Handler {
    pub path: Vec<String>,
    pub input_type: Type,
    pub format: HandlerInputFormat,
    pub method: Method,
    pub url: Option<String>,
    pub ignore_prefix: bool,
    #[serde(skip)] #[educe(Debug(ignore))]
    pub call: &'static dyn Next,
}

