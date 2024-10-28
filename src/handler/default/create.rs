use key_path::path;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::create::create_internal;
use crate::request::Request;

pub async fn create(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = CREATE | SINGLE | ENTRY;
    let value = request.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let binding = request.body_value();
        let input = binding.as_dictionary().unwrap();
        let create = input.get("create");
        let include = input.get("include");
        let select = input.get("select");
        Ok(create_internal(ctx.clone(), request.clone(), create, include, select, model, &path!["create"], action).await?)
    }).await?;
    Ok(Response::data(value))
}
