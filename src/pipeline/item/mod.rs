use std::future::Future;
use std::sync::Arc;
use educe::Educe;
use crate::arguments::Arguments;
use crate::pipeline::ctx::Ctx;
use crate::result::Result;
use futures_util::future::BoxFuture;

#[derive(Educe)]
#[educe(Debug)]
pub struct Item {
    pub path: Vec<String>,
    #[educe(Debug(ignore))]
    pub(crate) call: Arc<dyn Call>,
}

pub trait Call: Send + Sync {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, Result<Ctx>>;
}

impl<F, Fut> Call for F where
        F: Fn(Arguments, Ctx) -> Fut + Sync + Send,
        Fut: Future<Output = Result<Ctx>> + Send + 'static {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, Result<Ctx>> {
        Box::pin(self(args, ctx))
    }
}
