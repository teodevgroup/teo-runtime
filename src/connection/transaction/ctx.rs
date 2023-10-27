use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use key_path::KeyPath;
use maplit::btreemap;
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

    pub fn new(connection_ctx: connection::Ctx) -> Self {
        Self {
            inner: Arc::new(CtxInner {
                connection_ctx,
                transactions: Mutex::new(btreemap!{})
            })
        }
    }

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

    fn set_transaction_for_model(&self, model: &Model, transaction: Arc<dyn Transaction>) {
        self.set_transaction_for_namespace_path(&model.namespace_path(), transaction)
    }

    fn set_transaction_for_namespace(&self, namespace: &Namespace, transaction: Arc<dyn Transaction>) {
        self.set_transaction_for_namespace_path(&namespace.path(), transaction)
    }

    fn set_transaction_for_namespace_path(&self, path: &Vec<&str>, transaction: Arc<dyn Transaction>) {
        self.inner.transactions.lock().unwrap().insert(
            path.iter().map(ToString::to_string).collect(),
            transaction
        );
    }

    pub(crate) fn transaction_for_model(&self, model: &Model) -> Option<Arc<dyn Transaction>> {
        self.transaction_for_namespace_path(&model.namespace_path())
    }

    fn transaction_for_namespace(&self, namespace: &Namespace) -> Option<Arc<dyn Transaction>> {
        self.transaction_for_namespace_path(&namespace.path())
    }

    fn transaction_for_namespace_path(&self, path: &Vec<&str>) -> Option<Arc<dyn Transaction>> {
        let path: Vec<String> = path.iter().map(ToString::to_string).collect();
        self.inner.transactions.lock().unwrap().get(&path).cloned()
    }

    async fn transaction_for_model_or_create(&self, model: &Model) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&model.namespace_path()) {
            Ok(transaction)
        } else {
            self.connection_for_model(model).unwrap().transaction().await
        }
    }

    async fn transaction_for_model_or_no_transaction(&self, model: &Model) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&model.namespace_path()) {
            Ok(transaction)
        } else {
            self.connection_for_model(model).unwrap().no_transaction().await
        }
    }

    async fn transaction_for_namespace_or_create(&self, namespace: &Namespace) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&namespace.path()) {
            Ok(transaction)
        } else {
            self.connection_for_namespace(namespace).unwrap().transaction().await
        }
    }

    async fn transaction_for_namespace_or_no_transaction(&self, namespace: &Namespace) -> Result<Arc<dyn Transaction>> {
        if let Some(transaction) = self.transaction_for_namespace_path(&namespace.path()) {
            Ok(transaction)
        } else {
            self.connection_for_namespace(namespace).unwrap().no_transaction().await
        }
    }

    pub async fn run_transaction<F, Fut, C, R>(&self, models: Vec<&'static Model>, f: F) -> Result<R> where
        F: Fn(C) -> Fut,
        C: for <'a> From<&'a Ctx>,
        Fut: Future<Output = crate::path::Result<R>> {
        for model in models {
            let transaction = self.transaction_for_model_or_create(model).await?;
            self.set_transaction_for_model(model, transaction);
        }
        let result = f(self.into()).await;
        self.commit().await?;
        Ok(result?)
    }

    async fn commit(&self) -> Result<()> {
        for transaction in self.inner.transactions.lock().unwrap().values() {
            if transaction.is_transaction() {
                transaction.commit().await?;
            }
        }
        Ok(())
    }

    // database methods

    pub async fn find_unique<T: From<model::Object>>(&self, model: &'static Model, finder: &Value, req_ctx: Option<request::Ctx>, path: KeyPath) -> crate::path::Result<Option<T>> {
        match self.find_unique_internal(model, finder, false, CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx, path).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_first<T: From<model::Object>>(&self, model: &'static Model, finder: &Value, req_ctx: Option<request::Ctx>, path: KeyPath) -> crate::path::Result<Option<T>> {
        match self.find_first_internal(model, finder, false, CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx, path).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_many<T: From<model::Object>>(&self, model: &'static Model, finder: &Value, req_ctx: Option<request::Ctx>, path: KeyPath) -> crate::path::Result<Vec<T>> {
        match self.find_many_internal(model, finder, false, CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx, path).await {
            Ok(results) => Ok(results.iter().map(|item| item.clone().into()).collect()),
            Err(err) => Err(err),
        }
    }

    pub async fn find_unique_internal(&self, model: &'static Model, finder: &Value, ignore_select_and_include: bool, action: Action, req_ctx: Option<request::Ctx>, path: KeyPath) -> crate::path::Result<Option<model::Object>> {
        let transaction = self.transaction_for_model_or_no_transaction(model).await?;
        transaction.find_unique(model, finder, ignore_select_and_include, action, self.clone(), req_ctx, path).await
    }

    pub async fn find_first_internal(&self, model: &'static Model, finder: &Value, ignore_select_and_include: bool, action: Action, req_ctx: Option<request::Ctx>, path: KeyPath) -> crate::path::Result<Option<model::Object>> {
        let transaction = self.transaction_for_model_or_no_transaction(model).await?;
        let mut finder = finder.as_dictionary().clone().unwrap().clone();
        finder.insert("take".to_string(), 1.into());
        let finder = Value::Dictionary(finder);
        let result = transaction.find_many(model, &finder, ignore_select_and_include, action, self.clone(), req_ctx, path).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result.get(0).unwrap().clone()))
        }
    }

    pub async fn find_many_internal(&self, model: &'static Model, finder: &Value, ignore_select_and_include: bool, action: Action, req_ctx: Option<request::Ctx>, path: KeyPath) -> crate::path::Result<Vec<model::Object>> {
        let transaction = self.transaction_for_model_or_no_transaction(model).await?;
        transaction.find_many(model, finder, ignore_select_and_include, action, self.clone(), req_ctx, path).await
    }

    pub async fn batch<F, Fut>(&self, model: &'static Model, finder: &Value, action: Action, req_ctx: Option<request::Ctx>, path: KeyPath, f: F) -> Result<()> where
        F: Fn(model::Object) -> Fut,
        Fut: Future<Output = Result<()>> {
        let batch_size: usize = 200;
        let mut index: usize = 0;
        loop {
            let mut batch_finder = finder.clone();
            batch_finder.as_dictionary_mut().unwrap().insert("skip".to_owned(), (index * batch_size).into());
            batch_finder.as_dictionary_mut().unwrap().insert("take".to_owned(), batch_size.into());
            let results = self.find_many_internal(model, &batch_finder, true, action, req_ctx.clone(), path.clone()).await?;
            for result in results.iter() {
                f(result.clone()).await?;
            }
            if results.len() < batch_size {
                return Ok(());
            }
            index += 1;
        }
    }

    pub async fn count(&self, model: &'static Model, finder: &Value, path: KeyPath) -> crate::path::Result<usize> {
        let transaction = self.transaction_for_model_or_no_transaction(model).await?;
        transaction.count(model, finder, self.clone(), path).await
    }

    pub async fn aggregate(&self, model: &'static Model, finder: &Value, path: KeyPath) -> crate::path::Result<Value> {
        let transaction = self.transaction_for_model_or_no_transaction(model).await?;
        transaction.aggregate(model, finder, self.clone(), path).await
    }

    pub async fn group_by(&self, model: &'static Model, finder: &Value, path: KeyPath) -> crate::path::Result<Value> {
        let transaction = self.transaction_for_model_or_no_transaction(model).await?;
        transaction.group_by(model, finder, self.clone(), path).await
    }

    // MARK: - Create an object

    pub fn new_object(&self, model: &'static Model, action: Action, req_ctx: Option<request::Ctx>) -> Result<model::Object> {
        Ok(model::Object::new(req_ctx, self.clone(), model, action))
    }

    pub async fn new_object_with_teon_and_path<'a>(&self, model: &'static Model, initial: &Value, path: &KeyPath, action: Action, req_ctx: Option<request::Ctx>) -> Result<model::Object> {
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

impl From<&Ctx> for Ctx {

    fn from(value: &Ctx) -> Self {
        value.clone()
    }
}