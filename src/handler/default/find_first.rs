use key_path::path;
use crate::value::Value;
use crate::request::Request;
use crate::response::Response;
use crate::action::action::*;

pub async fn find_first(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = FIND | SINGLE | ENTRY;
    let result = request.transaction_ctx().find_first_internal(
        model,
        request.body_value()?,
        false,
        action,
        Some(request.clone()),
        path![],
    ).await?;
    match result {
        None => Ok(Response::data(Value::Null)),
        Some(obj) => {
            let obj_data = obj.to_teon_internal(&path!["data"]).await?;
            Ok(Response::data(obj_data))
        }
    }
}

