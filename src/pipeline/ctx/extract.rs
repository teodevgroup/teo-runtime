use crate::value::Value;
use crate::pipeline::Ctx;

pub trait ExtractFromPipelineCtx {
    fn extract( ctx: &Ctx) -> Self;
}

impl ExtractFromPipelineCtx for Ctx {
    fn extract(ctx: &Ctx) -> Self {
        ctx.clone()
    }
}

impl ExtractFromPipelineCtx for Value {
    fn extract(ctx: &Ctx) -> Self {
        ctx.value().clone()
    }
}
