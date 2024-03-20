pub mod transform;
pub mod validator;
pub mod callback;
pub mod compare;

use std::future::Future;
use std::sync::Arc;
use educe::Educe;
use crate::arguments::Arguments;
use crate::pipeline::ctx::Ctx;
use teo_result::Result;
use futures_util::future::BoxFuture;
use serde::Serialize;
use teo_parser::r#type::Type;
use crate::value::Value;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
pub struct Item {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
}

pub trait Call: Send + Sync {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, Result<Value>>;
}

impl<F, Fut> Call for F where
        F: Fn(Arguments, Ctx) -> Fut + Sync + Send,
        Fut: Future<Output = Result<Value>> + Send + 'static {
    fn call(&self, args: Arguments, ctx: Ctx) -> BoxFuture<'static, Result<Value>> {
        Box::pin(self(args, ctx))
    }
}

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct BoundedItem {
    pub path: Vec<String>,
    pub arguments: Arguments,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
    pub(crate) cast_output_type: Option<Type>,
}

impl BoundedItem {

    pub(crate) async fn call(&self, args: Arguments, ctx: Ctx) -> Result<Value> {
        self.call.call(args, ctx).await
    }
}