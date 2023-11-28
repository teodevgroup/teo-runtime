use teo_teon::Value;
use crate::pipeline::Ctx;

pub trait ExtractFromPipelineCtx {
    fn extract(ctx: &Ctx) -> Self;
}

impl ExtractFromPipelineCtx for Value {
    fn extract(ctx: &Ctx) -> Self {
        if let Some(teon) = ctx.value().as_teon() {
            teon.clone()
        } else {
            panic!("cannot extract value from pipeline ctx")
        }
    }
}