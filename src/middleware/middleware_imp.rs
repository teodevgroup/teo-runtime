use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::middleware::Middleware;
use crate::middleware::next::{Next, NextImp};
use crate::request::Request;
use crate::request::extract::ExtractFromRequest;
use crate::response::Response;

pub trait MiddlewareImp: Send + Sync {
    fn call(&self, request: Request, next: Next) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<F, Fut> MiddlewareImp for F where
    F: Fn(Request, Next) -> Fut + Sync + Send,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: Next) -> BoxFuture<'static, teo_result::Result<Response>> {
        Box::pin(self(request, next))
    }
}

pub(crate) fn empty_middleware() -> Middleware {
    Middleware::new(|request: Request, next: Next| async move {
        next.call(request).await
    })
}

pub(crate) fn combine_middleware(middlewares: Vec<Middleware>) -> Middleware {
    match middlewares.len() {
        0 => empty_middleware(),
        1 => middlewares.first().unwrap().clone(),
        2 => join_middleware(middlewares.get(1).unwrap().clone(), middlewares.get(0).unwrap().clone()),
        _ => {
            let inner_most = middlewares.first().unwrap().clone();
            let mut result = join_middleware(middlewares.get(1).unwrap().clone(), inner_most);
            for (index, middleware) in middlewares.iter().enumerate() {
                if index >= 2 {
                    result = join_middleware(middleware.clone(), result);
                }
            }
            result
        }
    }
}

fn join_middleware(outer: Middleware, inner: Middleware) -> Middleware {
    Middleware::new(move |req: Request, next: Next| {
        let outer = outer.clone();
        let inner = inner.clone();
        async move {
            outer.call(req, Next::new(move |req: Request| {
                let inner = inner.clone();
                let next = next.clone();
                async move {
                    inner.call(req, next).await
                }
            })).await
        }
    })
}

pub trait MiddlewareArgument<A>: Send + Sync + 'static {
    fn call(&self, request: Request, next: Arc<dyn NextImp>) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<A0, F, Fut> MiddlewareArgument<(A0,)> for F where
    A0: ExtractFromRequest + Send + Sync,
    F: Fn(A0, Arc<dyn NextImp>) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: Arc<dyn NextImp>) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value: A0 = ExtractFromRequest::extract(&request);
        Box::pin(self(value, next))
    }
}

impl<A0, A1, F, Fut> MiddlewareArgument<(A0, A1)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1, Arc<dyn NextImp>) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: Arc<dyn NextImp>) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequest::extract(&request);
        let a1: A1 = ExtractFromRequest::extract(&request);
        Box::pin(self(a0, a1, next))
    }
}

impl<A0, A1, A2, F, Fut> MiddlewareArgument<(A0, A1, A2)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    A2: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1, A2, Arc<dyn NextImp>) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: Arc<dyn NextImp>) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequest::extract(&request);
        let a1: A1 = ExtractFromRequest::extract(&request);
        let a2: A2 = ExtractFromRequest::extract(&request);
        Box::pin(self(a0, a1, a2, next))
    }
}

impl<A0, A1, A2, A3, F, Fut> MiddlewareArgument<(A0, A1, A2, A3)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    A2: ExtractFromRequest + Send + Sync,
    A3: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1, A2, A3, Arc<dyn NextImp>) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request, next: Arc<dyn NextImp>) -> BoxFuture<'static, teo_result::Result<Response>> {
        let a0: A0 = ExtractFromRequest::extract(&request);
        let a1: A1 = ExtractFromRequest::extract(&request);
        let a2: A2 = ExtractFromRequest::extract(&request);
        let a3: A3 = ExtractFromRequest::extract(&request);
        Box::pin(self(a0, a1, a2, a3, next))
    }
}
