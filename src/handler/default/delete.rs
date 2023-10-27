use key_path::path;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::model::object::object::ErrorIfNotFound;

pub async fn delete(ctx: request::Ctx) -> crate::path::Result<Response> {
    let model = ctx.namespace().model_at_path(&ctx.handler_match().path()).unwrap();
    let action = DELETE | ENTRY | SINGLE;
    ctx.transaction_ctx().transaction_for_model_or_create(model).await?;
    let object = ctx.transaction_ctx().find_unique_internal(model, ctx.body(), true, action, Some(ctx), path![]).await.into_not_found_error(path![])?;
    object.delete_internal(path!["delete"]).await?;
    ctx.transaction_ctx().commit().await?;
    let json_data = object.to_json_internal(&path!["data"]).await.unwrap();
    Ok(Response::data(json_data)?)
}
