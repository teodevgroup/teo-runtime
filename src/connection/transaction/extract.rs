use crate::connection::transaction::Ctx;

pub trait ExtractFromTransactionCtx {
    fn extract(ctx: &Ctx) -> Self;
}