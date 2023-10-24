use std::sync::Arc;
use crate::connection::connection::Connection;
use crate::namespace::Namespace;

#[derive(Debug, Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>
}

#[derive(Debug)]
struct CtxInner {
    namespace: &'static Namespace,
    transaction: Arc<dyn Connection>,
}