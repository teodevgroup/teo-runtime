use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use teo_result::Result;
use crate::connection;
use crate::connection::connection::Connection;
use crate::connection::transaction::Transaction;
use crate::model::Model;
use crate::namespace::Namespace;

#[derive(Debug, Clone)]
pub struct Ctx {
    inner: Arc<CtxInner>
}

#[derive(Debug)]
struct CtxInner {
    connection_ctx: connection::Ctx,
    transactions: Mutex<BTreeMap<Vec<String>, Arc<dyn Transaction>>>
}

impl Ctx {

    pub fn namespace(&self) -> &Namespace {
        self.inner.connection_ctx.namespace()
    }

    pub fn connection_for_model(&self, model: &Model) -> Option<Arc<dyn Connection>> {
        self.inner.connection_ctx.connection_for_model(model)
    }

    pub fn connection_for_namespace(&self, namespace: &Namespace) -> Option<Arc<dyn Connection>> {
        self.inner.connection_ctx.connection_for_namespace(namespace)
    }

    fn connection_for_namespace_path(&self, path: &Vec<&str>) -> Option<Arc<dyn Connection>> {
        self.inner.connection_ctx.connection_for_namespace_path(path)
    }

    fn connections(&self) -> Vec<Arc<dyn Connection>> {
        self.inner.connection_ctx.connections()
    }

    pub fn set_transaction_for_model(&self, model: &Model, transaction: Arc<dyn Transaction>) {
        self.set_transaction_for_namespace_path(&model.namespace_path(), transaction)
    }

    pub fn set_transaction_for_namespace(&self, namespace: &Namespace, transaction: Arc<dyn Transaction>) {
        self.set_transaction_for_namespace_path(&namespace.path(), transaction)
    }

    pub fn set_transaction_for_namespace_path(&self, path: &Vec<&str>, transaction: Arc<dyn Transaction>) {
        self.inner.transactions.lock().unwrap().insert(
            path.iter().map(ToString::to_string).collect(),
            transaction
        );
    }

    pub fn transaction_for_model(&self, model: &Model) -> Option<Arc<dyn Transaction>> {
        self.transaction_for_namespace_path(&model.namespace_path())
    }

    pub fn transaction_for_namespace(&self, namespace: &Namespace) -> Option<Arc<dyn Transaction>> {
        self.transaction_for_namespace_path(&namespace.path())
    }

    fn transaction_for_namespace_path(&self, path: &Vec<&str>) -> Option<Arc<dyn Transaction>> {
        let path = path.iter().map(ToString::to_string).collect();
        self.inner.transactions.lock().unwrap().get(path).cloned()
    }

    pub fn transaction_for_model_or_create(&self, model: &Model) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&model.namespace_path()) {
            Ok(transaction)
        } else {
            self.connection_for_model(model).unwrap().transaction()
        }
    }

    pub fn transaction_for_model_or_no_transaction(&self, model: &Model) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&model.namespace_path()) {
            Ok(transaction)
        } else {
            self.connection_for_model(model).unwrap().no_transaction()
        }
    }

    pub fn transaction_for_namespace_or_create(&self, namespace: &Namespace) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&namespace.path()) {
            Ok(transaction)
        } else {
            self.connection_for_namespace(namespace).unwrap().transaction()
        }
    }

    pub fn transaction_for_namespace_or_no_transaction(&self, namespace: &Namespace) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&namespace.path()) {
            Ok(transaction)
        } else {
            self.connection_for_namespace(namespace).unwrap().no_transaction()
        }
    }
}