use key_path::path;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::handler::default::internal::create::create_internal;
use crate::handler::default::internal::update::update_internal;

pub async fn upsert(ctx: request::Ctx) -> crate::path::Result<Response> {
    let model = ctx.namespace().model_at_path(&ctx.handler_match().path()).unwrap();
    let action = UPSERT | SINGLE | ENTRY;
    ctx.transaction_ctx().transaction_for_model_or_create(model).await?;
    let find_result = ctx.transaction_ctx().find_unique_internal(model, ctx.body(), true, action, Some(ctx), path![]).await?;
    let include = ctx.body().get("include");
    let select = ctx.body().get("select");
    match find_result {
        Some(object) => {
            let update = ctx.body().get("update");
            let value = update_internal(object, update, include, select, &path![]).await?;
            ctx.transaction_ctx().commit().await?;
            Ok(Response::data(value)?)
        }
        None => {
            let create = ctx.body().get("update");
            let value = create_internal(ctx.transaction_ctx().clone(), ctx.clone(), create, include, select, model, &path![], action).await?;
            ctx.transaction_ctx().commit().await?;
            Ok(Response::data(value)?)
        }
    }
}
