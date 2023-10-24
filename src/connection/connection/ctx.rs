use std::collections::BTreeMap;
use std::sync::Arc;
use maplit::btreemap;
use crate::connection::connection::Connection;
use crate::namespace::Namespace;

#[derive(Debug, Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>
}

#[derive(Debug)]
struct CtxInner {
    namespace: &'static Namespace,
    connections: Arc<BTreeMap<Vec<String>, Arc<dyn Connection>>>,
}

impl Ctx {

    pub fn from_namespace(namespace: &'static Namespace) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                namespace,
                connections: Arc::new(retrieve_connections(namespace)),
            })
        }
    }
}

fn retrieve_connections(namespace: &Namespace) -> BTreeMap<Vec<String>, Arc<dyn Connection>> {
    let mut result = btreemap!{};
    if let Some((k, c)) = retrieve_connection(namespace) {
        result.insert(k, c);
    }
    for namespace in namespace.namespaces.values() {
        result.extend(retrieve_connections(namespace))
    }
    result
}

fn retrieve_connection(namespace: &Namespace) -> Option<(Vec<String>, Arc<dyn Connection>)> {
    if let Some(connection) = &namespace.connection {
        Some((namespace.path.clone(), connection.clone()))
    } else {
        None
    }
}