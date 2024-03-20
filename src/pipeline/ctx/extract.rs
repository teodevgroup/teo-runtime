use crate::value::Value;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;

pub trait ExtractFromPipelineCtx {
    fn extract(args: &Arguments, ctx: &Ctx) -> Self;
}

impl ExtractFromPipelineCtx for Ctx {
    fn extract(_: &Arguments, ctx: &Ctx) -> Self {
        ctx.clone()
    }
}

impl ExtractFromPipelineCtx for Value {
    fn extract(_: &Arguments, ctx: &Ctx) -> Self {
        ctx.value().clone()
    }
}

impl ExtractFromPipelineCtx for Arguments {
    fn extract(args: &Arguments, _: &Ctx) -> Self {
        args.clone()
    }
}