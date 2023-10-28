use std::future::Future;
use educe::Educe;
use futures_util::future::BoxFuture;
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use crate::request::ctx::Ctx;
use crate::response::Response;
use teo_result::Result;

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
    pub format: HandlerInputFormat,
    pub method: Method,
    pub url: Option<String>,
    pub ignore_prefix: bool,
    #[serde(skip)] #[educe(Debug(ignore))]
    pub call: &'static dyn Call,
}

pub trait Call: Send + Sync {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<Response>>;
}

impl<F, Fut> Call for F where
    F: Fn(Ctx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<Response>> {
        Box::pin(self(ctx))
    }
}
