use key_path::path;
use teo_teon::Value;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::create::create_internal;

pub async fn create(req_ctx: &request::Ctx) -> crate::path::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = CREATE | SINGLE | ENTRY;
    let value = req_ctx.transaction_ctx().run_transaction(vec![model], |ctx: transaction::Ctx| async move {
        let input = req_ctx.body().as_dictionary().unwrap();
        let create = input.get("create");
        let include = input.get("include");
        let select = input.get("select");
        Ok(create_internal(ctx.clone(), req_ctx.clone(), create, include, select, model, &path!["create"], action).await?)
    }).await?;
    Ok(Response::data(value)?)
}
