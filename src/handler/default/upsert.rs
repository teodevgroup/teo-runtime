use key_path::path;
use crate::value::Value;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::create::create_internal;
use crate::handler::default::internal::update::update_internal;
use crate::request::Request;

pub async fn upsert(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = UPSERT | SINGLE | ENTRY;
    let value: Value = request.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let find_result = ctx.find_unique_internal(model, request.body_value().as_ref(), true, action, Some(request.clone()), path![]).await?;
        let binding = request.body_value();
        let include = binding.get("include");
        let select = binding.get("select");
        match find_result {
            Some(object) => {
                let update = binding.get("update");
                Ok(update_internal(object, update, include, select, &path![]).await?)
            }
            None => {
                let create = binding.get("update");
                Ok(create_internal(ctx.clone(), request.clone(), create, include, select, model, &path![], action).await?)
            }
        }
    }).await?;
    Ok(Response::data(value))
}
