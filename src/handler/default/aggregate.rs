use key_path::path;
use crate::request::Request;
use crate::response::Response;

pub async fn aggregate(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let result = request.transaction_ctx().aggregate(model, request.body_value().as_ref(), path![]).await?;
    Ok(Response::data(result))
}
