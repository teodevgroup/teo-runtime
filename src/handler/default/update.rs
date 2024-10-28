use key_path::path;
use crate::value::Value;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::update::update_internal;
use crate::model::object::object::ErrorIfNotFound;
use crate::request::Request;

pub async fn update(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = UPDATE | ENTRY | SINGLE;
    let value: Value = request.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let object = ctx.find_unique_internal(model, request.body_value().as_ref(), true, action, Some(request.clone()), path![]).await.into_not_found_error(path![])?;
        let binding = request.body_value();
        let update = binding.get("update");
        let include = binding.get("include");
        let select = binding.get("select");
        Ok(update_internal(object, update, include, select, &path!["update"]).await?)
    }).await?;
    Ok(Response::data(value))
}
