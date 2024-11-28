use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::middleware::NextImp;
use crate::request::Request;
use crate::response::Response;

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