use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::request::Request;
use crate::response::Response;

pub trait NextImp: Send + Sync {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<F, Fut> NextImp for F where
    F: Fn(Request) -> Fut + Sync + Send,
    Fut: Future<Output =teo_result::Result<Response>> + Send + 'static {
    fn call(&self, ctx: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        Box::pin(self(ctx))
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Next {
    pub imp: Arc<dyn NextImp>,
}

impl Next {

    #[inline]
    pub fn new<F>(f: F) -> Self where F: NextImp + 'static {
        Self {
            imp: Arc::new(f),
        }
    }
}

impl NextImp for Next {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        self.imp.call(request)
    }
}