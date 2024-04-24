use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use key_path::KeyPath;
use crate::value::Value;
use crate::action::Action;
use teo_result::Result;
use crate::connection::connection::Connection;
use crate::{model, request};
use crate::connection::transaction;
use crate::model::Model;

#[async_trait]
pub trait Transaction: Send + Sync + Debug {

    // Migration (Setup database)

    async fn migrate(&self, models: Vec<&Model>,  dry_run: bool, reset_database: bool, silent: bool) -> Result<()>;

    // Purge (Clear database data)

    async fn purge(&self, models: Vec<&Model>) -> Result<()>;

    // Query database

    async fn query_raw(&self, value: &Value) -> Result<Value>;

    // Object manipulation

    async fn save_object(&self, object: &model::Object, path: KeyPath) -> teo_result::Result<()>;

    async fn delete_object(&self, object: &model::Object, path: KeyPath) -> teo_result::Result<()>;

    async fn find_unique(&self, model: &'static Model, finder: &Value, ignore_select_and_include: bool, action: Action, transaction_ctx: transaction::Ctx, req_ctx: Option<request::Ctx>, path: KeyPath) -> teo_result::Result<Option<model::Object>>;

    async fn find_many(&self, model: &'static Model, finder: &Value, ignore_select_and_include: bool, action: Action, transaction_ctx: transaction::Ctx, req_ctx: Option<request::Ctx>, path: KeyPath) -> teo_result::Result<Vec<model::Object>>;

    async fn count(&self, model: &'static Model, finder: &Value, transaction_ctx: transaction::Ctx, path: KeyPath) -> teo_result::Result<Value>;

    async fn count_objects(&self, model: &'static Model, finder: &Value, transaction_ctx: transaction::Ctx, path: KeyPath) -> teo_result::Result<usize>;

    async fn count_fields(&self, model: &'static Model, finder: &Value, transaction_ctx: transaction::Ctx, path: KeyPath) -> teo_result::Result<Value>;

    async fn aggregate(&self, model: &'static Model, finder: &Value, transaction_ctx: transaction::Ctx, path: KeyPath) -> teo_result::Result<Value>;

    async fn group_by(&self, model: &'static Model, finder: &Value, transaction_ctx: transaction::Ctx, path: KeyPath) -> teo_result::Result<Vec<Value>>;

    async fn sql(&self, model: &'static Model, sql: &str, transaction_ctx: transaction::Ctx) -> Result<Vec<Value>>;

    // Transaction

    fn is_committed(&self) -> bool;

    fn is_transaction(&self) -> bool;

    async fn commit(&self) -> Result<()>;

    async fn abort(&self) -> Result<()>;

    async fn spawn(&self) -> Result<Arc<dyn Transaction>>;
}
