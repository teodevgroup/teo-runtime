use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::request::Request;

pub async fn delete_many(request: Request) -> teo_result::Result<Response> {
    let action = DELETE | MANY | ENTRY;
    let (objects, count) = request.transaction_ctx().run_transaction(move |ctx: transaction::Ctx| {
        let request = request.clone();
        let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap().clone();
        async move {
            let objects = ctx.find_many_internal(&model, request.body_value()?, true, action, Some(request.clone()), path![]).await?;
            let mut count = 0;
            let mut ret_data: Vec<Value> = vec![];
            for (index, object) in objects.iter().enumerate() {
                object.delete_internal(path!["data", index]).await?;
                ret_data.push(object.to_teon_internal(&path!["data", index]).await?);
                count += 1;
            }
            Ok((ret_data, count))
        }
    }).await?;
    Ok(Response::data_meta(Value::Array(objects), teon!({"count": count})))
}
