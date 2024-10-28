use std::future::Future;
use futures_util::future::BoxFuture;
use crate::request::Request;
use crate::request::extract::ExtractFromRequest;
use crate::response::Response;

pub trait HandlerCtxArgument<A>: Send + Sync {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>>;
}

impl<F, Fut> HandlerCtxArgument<()> for F where
    F: Fn() -> Fut + Sync + Send + Clone,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, _request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        Box::pin(self())
    }
}

impl<A0, F, Fut> HandlerCtxArgument<(A0,)> for F where
    A0: ExtractFromRequest + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value: A0 = ExtractFromRequest::extract(&request);
        Box::pin(self(value))
    }
}

impl<A0, A1, F, Fut> HandlerCtxArgument<(A0, A1)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + Clone,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1))
    }
}

impl<A0, A1, A2, F, Fut> HandlerCtxArgument<(A0, A1, A2)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    A2: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + Clone,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        let value2: A2 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1, value2))
    }
}

impl<A0, A1, A2, A3, F, Fut> HandlerCtxArgument<(A0, A1, A2, A3)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    A2: ExtractFromRequest + Send + Sync,
    A3: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + Clone,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        let value2: A2 = ExtractFromRequest::extract(&request);
        let value3: A3 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1, value2, value3))
    }
}

impl<A0, A1, A2, A3, A4, F, Fut> HandlerCtxArgument<(A0, A1, A2, A3, A4)> for F where
    A0: ExtractFromRequest + Send + Sync,
    A1: ExtractFromRequest + Send + Sync,
    A2: ExtractFromRequest + Send + Sync,
    A3: ExtractFromRequest + Send + Sync,
    A4: ExtractFromRequest + Send + Sync,
    F: Fn(A0, A1, A2, A3, A4) -> Fut + Sync + Send + Clone,
    Fut: Future<Output = teo_result::Result<Response>> + Send + 'static {
    fn call(&self, request: Request) -> BoxFuture<'static, teo_result::Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        let value2: A2 = ExtractFromRequest::extract(&request);
        let value3: A3 = ExtractFromRequest::extract(&request);
        let value4: A4 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1, value2, value3, value4))
    }
}