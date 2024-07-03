use std::collections::BTreeMap;
use std::sync::Arc;
use maplit::btreemap;
use crate::connection::connection::Connection;
use crate::model::Model;
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

    pub fn namespace(&self) -> &'static Namespace {
        self.inner.namespace
    }

    pub fn connection_for_model(&self, model: &Model) -> Option<Arc<dyn Connection>> {
        self.connection_for_namespace_path(&model.namespace_path())
    }

    pub fn connection_for_namespace(&self, namespace: &Namespace) -> Option<Arc<dyn Connection>> {
        if let Some(connection) = namespace.connection() {
            Some(connection.clone())
        } else if let Some(reference) = namespace.connector_reference() {
            self.connection_for_namespace_path(&reference)
        } else {
            None
        }
    }

    pub(in crate::connection) fn connection_for_namespace_path(&self, path: &Vec<String>) -> Option<Arc<dyn Connection>> {
        let namespace = self.namespace().namespace_at_path(path).unwrap();
        if let Some(connection) = namespace.connection() {
            Some(connection.clone())
        } else if let Some(reference) = namespace.connector_reference() {
            self.connection_for_namespace_path(reference)
        } else {
            None
        }
    }

    pub(in crate::connection) fn connections(&self) -> Vec<Arc<dyn Connection>> {
        self.inner.connections.values().map(Clone::clone).collect()
    }

    pub fn connections_iter(&self) -> &BTreeMap<Vec<String>, Arc<dyn Connection>> {
        self.inner.connections.as_ref()
    }
}

fn retrieve_connections(namespace: &Namespace) -> BTreeMap<Vec<String>, Arc<dyn Connection>> {
    let mut result = btreemap!{};
    if let Some((k, c)) = retrieve_connection(namespace) {
        result.insert(k, c);
    }
    for namespace in namespace.namespaces().values() {
        result.extend(retrieve_connections(namespace))
    }
    result
}

fn retrieve_connection(namespace: &Namespace) -> Option<(Vec<String>, Arc<dyn Connection>)> {
    if let Some(connection) = namespace.connection() {
        Some((namespace.path().clone(), connection.clone()))
    } else {
        None
    }
}