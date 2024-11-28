use key_path::path;
use crate::value::Value;
use crate::response::Response;
use crate::action::action::*;
use crate::request::Request;

pub async fn find_unique(request: Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match()?.path()).unwrap().clone();
    let action = FIND | SINGLE | ENTRY;
    let result = request.transaction_ctx().find_unique_internal(
        &model,
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

