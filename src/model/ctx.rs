use crate::connection::transaction;
use crate::model::Model;

pub struct Ctx {
    pub transaction_ctx: transaction::Ctx,
    pub model: &'static Model,
}