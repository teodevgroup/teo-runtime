use std::collections::BTreeMap;
use std::sync::Arc;
use crate::connection::connection::Connection;
use crate::namespace::Namespace;

#[derive(Debug)]
pub struct Ctx {
    namespace: &'static Namespace,
    connections: Arc<BTreeMap<Vec<String>, Arc<dyn Connection>>>,
}