use std::future::Future;
use futures_util::future::BoxFuture;
use crate::middleware::next::Next;
use crate::request::Request;
use crate::request::extract::ExtractFromRequest;
use crate::response::Response;

pub trait Middleware: Send + Sync {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(Request, &'static dyn Next) -> Fut + Sync + Send,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        Box::pin(self(request, next))
    }
}

pub(crate) fn empty_middleware() -> &'static dyn Middleware {
    Box::leak(Box::new(|request: Request, next: &'static dyn Next| async move {
        next.call(request).await
    }))
}

pub(crate) fn combine_middleware(middlewares: Vec<&'static dyn Middleware>) -> &'static dyn Middleware {
    match middlewares.len() {
        0 => empty_middleware(),
        1 => *middlewares.first().unwrap(),
        2 => join_middleware(*middlewares.get(1).unwrap(), *middlewares.get(0).unwrap()),
        _ => {
            let inner_most = *middlewares.first().unwrap();
            let mut result = join_middleware(*middlewares.get(1).unwrap(), inner_most);
            for (index, middleware) in middlewares.iter().enumerate() {
                if index >= 2 {
                    result = join_middleware(*middleware, result);
                }
            }
            result
        }
    }
}

fn join_middleware(outer: &'static dyn Middleware, inner: &'static dyn Middleware) -> &'static dyn Middleware {
    Box::leak(Box::new(move |request: Request, next: &'static dyn Next| async move {
        outer.call(request, Box::leak(Box::new(move |request: Request| async move {
            inner.call(request, next).await
        }))).await
    }))
}

pub trait MiddlewareArgument<A>: Send + Sync + 'static {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<A0, F, Fut> MiddlewareArgument<(A0,)> for F where
    A0: for<'a> ExtractFromRequest<'a> + Send + Sync,
    F: Fn(A0, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value: A0 = ExtractFromRequest::extract(&request);
        Box::pin(self(value, next))
    }
}

impl<A0, A1, F, Fut> MiddlewareArgument<(A0, A1)> for F where
    A0: for<'a> ExtractFromRequest<'a> + Send + Sync,
    A1: for<'a> ExtractFromRequest<'a> + Send + Sync,
    F: Fn(A0, A1, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequest::extract(&request);
        let a1: A1 = ExtractFromRequest::extract(&request);
        Box::pin(self(a0, a1, next))
    }
}

impl<A0, A1, A2, F, Fut> MiddlewareArgument<(A0, A1, A2)> for F where
    A0: for<'a> ExtractFromRequest<'a> + Send + Sync,
    A1: for<'a> ExtractFromRequest<'a> + Send + Sync,
    A2: for<'a> ExtractFromRequest<'a> + Send + Sync,
    F: Fn(A0, A1, A2, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequest::extract(&request);
        let a1: A1 = ExtractFromRequest::extract(&request);
        let a2: A2 = ExtractFromRequest::extract(&request);
        Box::pin(self(a0, a1, a2, next))
    }
}

impl<A0, A1, A2, A3, F, Fut> MiddlewareArgument<(A0, A1, A2, A3)> for F where
    A0: for<'a> ExtractFromRequest<'a> + Send + Sync,
    A1: for<'a> ExtractFromRequest<'a> + Send + Sync,
    A2: for<'a> ExtractFromRequest<'a> + Send + Sync,
    A3: for<'a> ExtractFromRequest<'a> + Send + Sync,
    F: Fn(A0, A1, A2, A3, &'static dyn Next) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: &'static dyn Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequest::extract(&request);
        let a1: A1 = ExtractFromRequest::extract(&request);
        let a2: A2 = ExtractFromRequest::extract(&request);
        let a3: A3 = ExtractFromRequest::extract(&request);
        Box::pin(self(a0, a1, a2, a3, next))
    }
}
