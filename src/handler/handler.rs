use std::future::Future;
use std::sync::Arc;
use educe::Educe;
use futures_util::future::BoxFuture;
use serde::Serialize;
use crate::request::ctx::Ctx;
use crate::response::Response;
use crate::result::Result;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
pub struct Handler {
    pub path: Vec<String>,
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
