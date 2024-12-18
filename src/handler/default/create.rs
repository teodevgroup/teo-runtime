use key_path::path;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::create::create_internal;
use crate::request::Request;

pub async fn create(request: Request) -> teo_result::Result<Response> {
    let action = CREATE | SINGLE | ENTRY;
    let value = request.transaction_ctx().run_transaction(move |ctx: transaction::Ctx| {
        let request = request.clone();
        async move {
            let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match()?.path()).unwrap().clone();
            let input = request.body_value()?.as_dictionary().unwrap();
            let create = input.get("create");
            let include = input.get("include");
            let select = input.get("select");
            Ok(create_internal(ctx.clone(), request.clone(), create, include, select, &model, &path!["create"], action).await?)
        }
    }).await?;
    Ok(Response::data(value))
}
