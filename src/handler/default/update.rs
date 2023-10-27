use key_path::path;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::handler::default::internal::update::update_internal;
use crate::model::object::object::ErrorIfNotFound;

pub async fn update(ctx: request::Ctx) -> crate::path::Result<Response> {
    let model = ctx.namespace().model_at_path(&ctx.handler_match().path()).unwrap();
    let action = UPDATE | ENTRY | SINGLE;
    ctx.transaction_ctx().transaction_for_model_or_create(model).await?;
    let object = ctx.transaction_ctx().find_unique_internal(model, ctx.body(), true, action, Some(ctx), path![]).await.into_not_found_error(path![])?;
    let update = ctx.body().get("update");
    let include = ctx.body().get("include");
    let select = ctx.body().get("select");
    let update_result = update_internal(object, update, include, select, &path!["update"]).await?;
    ctx.transaction_ctx().commit().await?;
    Ok(Response::data(update_result)?)
}
