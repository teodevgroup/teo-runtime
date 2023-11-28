use crate::request::Ctx;

pub trait ExtractFromRequestCtx {
    fn extract(ctx: &Ctx) -> Self;
}