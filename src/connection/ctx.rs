use std::collections::BTreeMap;
use std::sync::Arc;
use crate::connection::transaction::Transaction;

#[derive(Debug)]
pub struct Ctx {
    transactions: Arc<BTreeMap<Vec<String>, Arc<dyn Transaction>>>,
}