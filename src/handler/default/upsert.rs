use key_path::path;
use crate::value::Value;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::create::create_internal;
use crate::handler::default::internal::update::update_internal;

pub async fn upsert(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.request().handler_match().unwrap().path()).unwrap();
    let action = UPSERT | SINGLE | ENTRY;
    let value: Value = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let find_result = ctx.find_unique_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await?;
        let include = req_ctx.body().get("include");
        let select = req_ctx.body().get("select");
        match find_result {
            Some(object) => {
                let update = req_ctx.body().get("update");
                Ok(update_internal(object, update, include, select, &path![]).await?)
            }
            None => {
                let create = req_ctx.body().get("update");
                Ok(create_internal(ctx.clone(), req_ctx.clone(), create, include, select, model, &path![], action).await?)
            }
        }
    }).await?;
    Ok(Response::data(value))
}
