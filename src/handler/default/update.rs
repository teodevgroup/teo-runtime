use key_path::path;
use crate::value::Value;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::update::update_internal;
use crate::model::object::object::ErrorIfNotFound;

pub async fn update(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.request().handler_match().unwrap().path()).unwrap();
    let action = UPDATE | ENTRY | SINGLE;
    let value: Value = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let object = ctx.find_unique_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await.into_not_found_error(path![])?;
        let update = req_ctx.body().get("update");
        let include = req_ctx.body().get("include");
        let select = req_ctx.body().get("select");
        Ok(update_internal(object, update, include, select, &path!["update"]).await?)
    }).await?;
    Ok(Response::data(value))
}
