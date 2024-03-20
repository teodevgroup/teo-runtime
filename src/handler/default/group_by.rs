use key_path::path;
use crate::value::Value;
use crate::request;
use crate::response::Response;

pub async fn group_by(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let result = req_ctx.transaction_ctx().group_by(model, req_ctx.body(), path![]).await?;
    Ok(Response::data(Value::Array(result)))
}
