use key_path::path;
use crate::value::Value;
use crate::request;
use crate::response::Response;
use crate::action::action::*;

pub async fn find_first(ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = ctx.namespace().model_at_path(&ctx.handler_match().path()).unwrap();
    let action = FIND | SINGLE | ENTRY;
    let result = ctx.transaction_ctx().find_first_internal(
        model,
        ctx.body(),
        false,
        action,
        Some(ctx.clone()),
        path![],
    ).await?;
    match result {
        None => Ok(Response::data(Value::Null)),
        Some(obj) => {
            let obj_data = obj.to_teon_internal(&path!["data"]).await?;
            Ok(Response::data(obj_data))
        }
    }
}

