use std::future::Future;
use educe::Educe;
use futures_util::future::BoxFuture;
use serde::Serialize;
use crate::request::ctx::Ctx;
use crate::response::Response;
use teo_result::Result;

#[derive(Debug, Serialize)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
pub struct Handler {
    pub path: Vec<String>,
    pub method: Method,
    pub url: Option<String>,
    pub ignore_namespace: bool,
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
