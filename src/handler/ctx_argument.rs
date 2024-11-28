use std::future::Future;
use futures_util::future::BoxFuture;
use teo_result::Result;
use crate::request::Request;
use crate::request::extract::ExtractFromRequest;
use crate::response::Response;

pub trait HandlerCtxArgument<'a, A: 'a>: Send + Sync {
    fn call(&self, request: &'a Request) -> BoxFuture<'a, Result<Response>>;
}

impl<'a, F, Fut> HandlerCtxArgument<'a, ()> for F where
    F: Fn() -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'a {
    fn call(&self, _request: &'a Request) -> BoxFuture<'a, Result<Response>> {
        Box::pin(self())
    }
}

impl<'a, A0, F, Fut> HandlerCtxArgument<'a, (A0,)> for F where
    A0: ExtractFromRequest<'a> + Send + Sync + 'a,
    F: Fn(A0) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'a {
    fn call(&self, request: &'a Request) -> BoxFuture<'a, Result<Response>> {
        let value: A0 = ExtractFromRequest::extract(&request);
        Box::pin(self(value))
    }
}

impl<'a, A0, A1, F, Fut> HandlerCtxArgument<'a, (A0, A1)> for F where
    A0: ExtractFromRequest<'a> + Send + Sync + 'a,
    A1: ExtractFromRequest<'a> + Send + Sync + 'a,
    F: Fn(A0, A1) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'a {
    fn call(&self, request: &'a Request) -> BoxFuture<'a, Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1))
    }
}

impl<'a, A0, A1, A2, F, Fut> HandlerCtxArgument<'a, (A0, A1, A2)> for F where
    A0: ExtractFromRequest<'a> + Send + Sync + 'a,
    A1: ExtractFromRequest<'a> + Send + Sync + 'a,
    A2: ExtractFromRequest<'a> + Send + Sync + 'a,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'a {
    fn call(&self, request: &'a Request) -> BoxFuture<'a, Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        let value2: A2 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1, value2))
    }
}

impl<'a, A0, A1, A2, A3, F, Fut> HandlerCtxArgument<'a, (A0, A1, A2, A3)> for F where
    A0: ExtractFromRequest<'a> + Send + Sync + 'a,
    A1: ExtractFromRequest<'a> + Send + Sync + 'a,
    A2: ExtractFromRequest<'a> + Send + Sync + 'a,
    A3: ExtractFromRequest<'a> + Send + Sync + 'a,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'a {
    fn call(&self, request: &'a Request) -> BoxFuture<'a, Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        let value2: A2 = ExtractFromRequest::extract(&request);
        let value3: A3 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1, value2, value3))
    }
}

impl<'a, A0, A1, A2, A3, A4, F, Fut> HandlerCtxArgument<'a, (A0, A1, A2, A3, A4)> for F where
    A0: ExtractFromRequest<'a> + Send + Sync + 'a,
    A1: ExtractFromRequest<'a> + Send + Sync + 'a,
    A2: ExtractFromRequest<'a> + Send + Sync + 'a,
    A3: ExtractFromRequest<'a> + Send + Sync + 'a,
    A4: ExtractFromRequest<'a> + Send + Sync + 'a,
    F: Fn(A0, A1, A2, A3, A4) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'a {
    fn call(&self, request: &'a Request) -> BoxFuture<'a, Result<Response>> {
        let value0: A0 = ExtractFromRequest::extract(&request);
        let value1: A1 = ExtractFromRequest::extract(&request);
        let value2: A2 = ExtractFromRequest::extract(&request);
        let value3: A3 = ExtractFromRequest::extract(&request);
        let value4: A4 = ExtractFromRequest::extract(&request);
        Box::pin(self(value0, value1, value2, value3, value4))
    }
}