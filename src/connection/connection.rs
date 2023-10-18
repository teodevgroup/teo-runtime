use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use crate::connection::transaction::Transaction;
use crate::result::Result;

#[async_trait]
pub trait Connection: Send + Sync + Debug {

    async fn transaction(&self) -> Result<Arc<dyn Transaction>>;
}