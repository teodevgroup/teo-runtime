use key_path::path;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::handler::default::internal::create::create_internal;

pub async fn create(ctx: request::Ctx) -> crate::path::Result<Response> {
    let model = ctx.namespace().model_at_path(&ctx.handler_match().path()).unwrap();
    let action = FIND | SINGLE | ENTRY;
    ctx.transaction_ctx().transaction_for_model_or_create(model).await?;
    let input = ctx.body().as_dictionary().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    let result = create_internal(ctx.transaction_ctx().clone(), ctx, create, include, select, model, &path!["create"], action).await;
    ctx.transaction_ctx().commit().await?;
    match result {
        Ok(result) => Ok(Response::data(result)?),
        Err(err) => Err(err),
    }
}
