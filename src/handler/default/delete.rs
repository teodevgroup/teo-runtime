use key_path::path;
use crate::value::Value;
use crate::request::Request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::model::object::object::ErrorIfNotFound;

pub async fn delete(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = DELETE | ENTRY | SINGLE;
    let value: Value = request.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let object = ctx.find_unique_internal(model, request.body_value()?, true, action, Some(request.clone()), path![]).await.into_not_found_error(path![])?;
        object.delete_internal(path!["delete"]).await?;
        Ok(object.to_teon_internal(&path!["data"]).await?)
    }).await?;
    Ok(Response::data(value))
}
