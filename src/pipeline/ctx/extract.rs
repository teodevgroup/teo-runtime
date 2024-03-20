use crate::value::Value;
use crate::arguments::Arguments;
use crate::object;
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
        if let Some(teon) = ctx.value().as_teon() {
            teon.clone()
        } else {
            panic!("cannot extract value from pipeline ctx")
        }
    }
}

impl ExtractFromPipelineCtx for object::Object {
    fn extract(_: &Arguments, ctx: &Ctx) -> Self {
        ctx.value().clone()
    }
}

impl ExtractFromPipelineCtx for Arguments {
    fn extract(args: &Arguments, _: &Ctx) -> Self {
        args.clone()
    }
}