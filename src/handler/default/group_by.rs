use key_path::path;
use crate::value::Value;
use crate::request::Request;
use crate::response::Response;

pub async fn group_by(request: Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match()?.path()).unwrap().clone();
    let result = request.transaction_ctx().group_by(&model, request.body_value()?, path![]).await?;
    Ok(Response::data(Value::Array(result)))
}
