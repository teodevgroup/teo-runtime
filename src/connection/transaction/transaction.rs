use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use teo_teon::Value;
use crate::action::Action;
use teo_result::Result;
use crate::connection::connection::Connection;
use crate::{model, request};
use crate::connection::transaction;
use crate::model::Model;

#[async_trait]
pub trait Transaction: Send + Sync + Debug {

    // Migration (Setup database)

    async fn migrate(&self, models: Vec<&Model>, reset_database: bool) -> Result<()>;

    // Purge (Clear database data)

    async fn purge(&self, models: Vec<&Model>) -> Result<()>;

    // Query database

    async fn query_raw(&self, value: &Value) -> Result<Value>;

    // Object manipulation

    async fn save_object(&self, object: &model::Object) -> Result<()>;

    async fn delete_object(&self, object: &model::Object) -> Result<()>;

    async fn find_unique(&self, model: &Model, finder: &Value, ignore_select_and_include: bool, action: Action, transaction_ctx: transaction::Ctx, req_ctx: Option<request::Ctx>) -> Result<Option<model::Object>>;

    async fn find_many(&self, model: &Model, finder: &Value, ignore_select_and_include: bool, action: Action, transaction_ctx: transaction::Ctx, req_ctx: Option<request::Ctx>) -> Result<Vec<model::Object>>;

    async fn count(&self, model: &Model, finder: &Value, transaction_ctx: transaction::Ctx) -> Result<usize>;

    async fn aggregate(&self, model: &Model, finder: &Value, transaction_ctx: transaction::Ctx) -> Result<Value>;

    async fn group_by(&self, model: &Model, finder: &Value, transaction_ctx: transaction::Ctx) -> Result<Value>;

    // Transaction

    async fn is_committed(&self) -> bool;

    async fn commit(&self) -> Result<()>;

    async fn spawn(&self) -> Result<Arc<dyn Transaction>>;
}