use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use key_path::KeyPath;
use teo_result::Result;
use teo_teon::Value;
use crate::{connection, model, request};
use crate::connection::connection::Connection;
use crate::connection::transaction::Transaction;
use crate::model::Model;
use crate::namespace::Namespace;
use crate::action::*;
use crate::action::action::{CODE_AMOUNT, CODE_NAME, CODE_POSITION, CREATE, SINGLE};

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

    pub fn namespace(&self) -> &'static Namespace {
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
        let path: Vec<String> = path.iter().map(ToString::to_string).collect();
        self.inner.transactions.lock().unwrap().get(&path).cloned()
    }

    pub async fn transaction_for_model_or_create(&self, model: &Model) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&model.namespace_path()) {
            Ok(transaction)
        } else {
            self.connection_for_model(model).unwrap().transaction().await
        }
    }

    pub async fn transaction_for_model_or_no_transaction(&self, model: &Model) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&model.namespace_path()) {
            Ok(transaction)
        } else {
            self.connection_for_model(model).unwrap().no_transaction().await
        }
    }

    pub async fn transaction_for_namespace_or_create(&self, namespace: &Namespace) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&namespace.path()) {
            Ok(transaction)
        } else {
            self.connection_for_namespace(namespace).unwrap().transaction().await
        }
    }

    pub async fn transaction_for_namespace_or_no_transaction(&self, namespace: &Namespace) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&namespace.path()) {
            Ok(transaction)
        } else {
            self.connection_for_namespace(namespace).unwrap().no_transaction().await
        }
    }

    // database methods

    pub async fn find_unique<T: From<model::Object>>(&self, model: &Model, finder: &Value, req_ctx: Option<request::Ctx>) -> Result<Option<T>> {
        match self.find_unique_internal(model, finder, false, CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_first<T: From<model::Object>>(&self, model: &Model, finder: &Value, req_ctx: Option<request::Ctx>) -> Result<Option<T>> {
        match self.find_first_internal(model, finder, false, CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_many<T: From<model::Object>>(&self, model: &Model, finder: &Value, req_ctx: Option<request::Ctx>) -> Result<Vec<T>> {
        match self.find_many_internal(model, finder, false, CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx).await {
            Ok(results) => Ok(results.iter().map(|item| item.clone().into()).collect()),
            Err(err) => Err(err),
        }
    }

    pub(crate) async fn find_unique_internal(&self, model: &Model, finder: &Value, ignore_select_and_include: bool, action: Action, req_ctx: Option<request::Ctx>) -> Result<Option<model::Object>> {
        let transaction = self.transaction_for_model_or_create(model).await?;
        transaction.find_unique(model, finder, ignore_select_and_include, action, req_ctx).await
    }

    pub(crate) async fn find_first_internal(&self, model: &Model, finder: &Value, ignore_select_and_include: bool, action: Action, req_ctx: Option<request::Ctx>) -> Result<Option<model::Object>> {
        let transaction = self.transaction_for_model_or_create(model).await?;
        let mut finder = finder.as_dictionary().clone().unwrap().clone();
        finder.insert("take".to_string(), 1.into());
        let finder = Value::Dictionary(finder);
        let result = transaction.find_many(model, &finder, ignore_select_and_include, action, req_ctx).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result.get(0).unwrap().clone()))
        }
    }

    pub(crate) async fn find_many_internal(&self, model: &Model, finder: &Value, mutation_mode: bool, action: Action, req_ctx: Option<request::Ctx>) -> Result<Vec<model::Object>> {
        let transaction = self.transaction_for_model_or_create(model).await?;
        transaction.find_many(model, finder, mutation_mode, action, req_ctx).await
    }

    pub(crate) async fn batch<'a, F, Fut>(&self, model: &'static Model, finder: &'a Value, action: Action, req_ctx: Option<request::Ctx>, f: F) -> Result<()> where
        F: Fn(model::Object) -> Fut,
        Fut: Future<Output = Result<()>> {
        let batch_size: usize = 200;
        let mut index: usize = 0;
        loop {
            let mut batch_finder = finder.clone();
            batch_finder.as_dictionary_mut().unwrap().insert("skip".to_owned(), (index * batch_size).into());
            batch_finder.as_dictionary_mut().unwrap().insert("take".to_owned(), batch_size.into());
            let results = self.find_many_internal(model, &batch_finder, true, action, req_ctx.clone()).await?;
            for result in results.iter() {
                f(result.clone()).await?;
            }
            if results.len() < batch_size {
                return Ok(());
            }
            index += 1;
        }
    }

    pub(crate) async fn count<'a>(&self, model: &'static Model, finder: &'a Value) -> Result<usize> {
        let transaction = self.transaction_for_model_or_create(model).await?;
        transaction.count(model, finder).await
    }

    pub(crate) async fn aggregate<'a>(&self, model: &'static Model, finder: &'a Value) -> Result<Value> {
        let transaction = self.transaction_for_model_or_create(model).await?;
        transaction.aggregate(model, finder).await
    }

    pub(crate) async fn group_by<'a>(&self, model: &'static Model, finder: &'a Value) -> Result<Value> {
        let transaction = self.transaction_for_model_or_create(model).await?;
        transaction.group_by(model, finder).await
    }

    // MARK: - Create an object

    pub(crate) fn new_object(&self, model: &'static Model, action: Action, req_ctx: Option<request::Ctx>) -> Result<model::Object> {
        Ok(model::Object::new(req_ctx, self.clone(), model, action))
    }

    pub(crate) async fn new_object_with_teon_and_path<'a>(&self, model: &'static Model, initial: &Value, path: &KeyPath, action: Action, req_ctx: Option<request::Ctx>) -> Result<model::Object> {
        let object = self.new_object(model, action, req_ctx)?;
        object.set_teon_with_path(initial, path).await?;
        Ok(object)
    }

    pub async fn create_object(&self, model: &'static Model, initial: impl Borrow<Value>, req_ctx: Option<request::Ctx>) -> Result<model::Object> {
        let object = self.new_object(model, CODE_NAME | CREATE | SINGLE | CODE_POSITION, req_ctx)?;
        object.set_teon(initial.borrow()).await?;
        Ok(object)
    }
}