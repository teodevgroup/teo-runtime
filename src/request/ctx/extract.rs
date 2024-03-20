use crate::value::Value;
use crate::request::Ctx;

pub trait ExtractFromRequestCtx {
    fn extract(ctx: &Ctx) -> Self;
}

impl ExtractFromRequestCtx for Value {
    fn extract(ctx: &Ctx) -> Self {
        ctx.body().clone()
    }
}