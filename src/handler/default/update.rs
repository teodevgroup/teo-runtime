use key_path::path;
use teo_teon::Value;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::update::update_internal;
use crate::model::object::object::ErrorIfNotFound;

pub async fn update(req_ctx: &request::Ctx) -> crate::path::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = UPDATE | ENTRY | SINGLE;
    let value: Value = req_ctx.transaction_ctx().run_transaction(vec![model], |ctx: transaction::Ctx| async move {
        let object = ctx.find_unique_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await.into_not_found_error(path![])?;
        let update = req_ctx.body().get("update");
        let include = req_ctx.body().get("include");
        let select = req_ctx.body().get("select");
        Ok(update_internal(object, update, include, select, &path!["update"]).await?)
    }).await?;
    Ok(Response::data(value)?)
}
