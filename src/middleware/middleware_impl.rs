use crate::middleware::middleware::Middleware;

pub struct MiddlewareImpl {
    pub middleware: &'static dyn Middleware,
}

impl MiddlewareImpl {

    pub fn new<F>(f: F) -> Self where F: Middleware + 'static {
        Self {
            middleware: unsafe { &*Box::into_raw(Box::new(f)) },
        }
    }
}