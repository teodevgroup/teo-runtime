use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::request::Request;

pub async fn copy_or_create(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = COPY | SINGLE | ENTRY;
    let value: Value = request.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let include = request.body_value()?.get("include");
        let select = request.body_value()?.get("select");
        let object = ctx.find_unique_internal(model, request.body_value()?, true, action, Some(request.clone()), path![]).await?;
        match object {
            Some(object) => {
                let copy = request.body_value()?.get("copy");
                let value = object.copied_value();
                let new = ctx.new_object_with_teon_and_path(model, &teon!({}), &path![], action, Some(request.clone())).await?;
                new.update_teon(&value).await?;
                if let Some(copy) = copy {
                    new.set_teon_with_path(copy, &path!["copy"]).await?;
                }
                new.save_with_session_and_path(&path!["copy"]).await?;
                let refreshed = new.refreshed(include, select).await?;
                refreshed.to_teon_internal(&path!["data"]).await
            }
            None => {
                let create = request.body_value()?.get("create");
                let new = ctx.new_object_with_teon_and_path(model, &teon!({}), &path![], action, Some(request.clone())).await?;
                if let Some(create) = create {
                    new.set_teon_with_path(create, &path!["create"]).await?;
                }
                new.save_with_session_and_path(&path!["create"]).await?;
                let refreshed = new.refreshed(include, select).await?;
                refreshed.to_teon_internal(&path!["data"]).await
            }
        }
    }).await?;
    Ok(Response::data(value))
}
