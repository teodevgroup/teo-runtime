use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::middleware::middleware_imp::MiddlewareImp;
use crate::middleware::next::Next;
use crate::request::Request;
use crate::response::Response;
use teo_result::Result;

#[derive(Clone)]
#[repr(transparent)]
pub struct Middleware {
    pub imp: Arc<dyn MiddlewareImp>,
}

impl Middleware {

    #[inline]
    pub fn new<F>(f: F) -> Self where F: MiddlewareImp + 'static {
        Self {
            imp: Arc::new(f),
        }
    }
}

impl MiddlewareImp for Middleware {
    fn call(&self, request: Request, next: Next) -> BoxFuture<'static, Result<Response>> {
        self.imp.call(request, next)
    }
}