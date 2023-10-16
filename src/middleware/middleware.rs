use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::middleware::next::Next;
use crate::request::ctx::Ctx;
use crate::response::Response;
use crate::result::Result;

pub trait Middleware: Send + Sync {
    fn call(&self, ctx: Ctx, next: Arc<dyn Next>) -> BoxFuture<'static, Result<Response>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(Ctx, Arc<dyn Next>) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Response>> + Send + 'static {
    fn call(&self, ctx: Ctx, next: Arc<dyn Next>) -> BoxFuture<'static, Result<Response>> {
        Box::pin(self(ctx, next))
    }
}

pub(crate) fn combine_middleware(middlewares: Vec<Arc<dyn Middleware>>) -> Arc<dyn Middleware> {
    match middlewares.len() {
        0 => Arc::new(|ctx: Ctx, next: Arc<dyn Next>| async move {
            next.call(ctx).await
        }),
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

fn join_middleware(outer: Arc<dyn Middleware>, inner: Arc<dyn Middleware>) -> Arc<dyn Middleware> {
    return Arc::new(move |ctx: Ctx, next: Arc<dyn Next>| async move {
        outer.call(ctx, Arc::new(move |ctx: Ctx| async move {
            inner.call(ctx, next).await
        })).await
    })
}