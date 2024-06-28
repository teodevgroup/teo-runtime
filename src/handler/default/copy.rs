use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::model::object::object::ErrorIfNotFound;

pub async fn copy(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = COPY | SINGLE | ENTRY;
    let value: Value = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let object = ctx.find_unique_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await?.into_not_found_error(path![])?;
        let copy = req_ctx.body().get("copy");
        let include = req_ctx.body().get("include");
        let select = req_ctx.body().get("select");
        let value = object.copied_value();
        let new = ctx.new_object_with_teon_and_path(model, &teon!({}), &path![], action, Some(req_ctx.clone())).await?;
        new.update_teon(&value).await?;
        if let Some(copy) = copy {
            new.set_teon_with_path(copy, &path!["copy"]).await?;
        }
        new.save_with_session_and_path(&path!["copy"]).await?;
        let refreshed = new.refreshed(include, select).await?;
        refreshed.to_teon_internal(&path!["data"]).await
    }).await?;
    Ok(Response::data(value))
}
