use key_path::path;
use teo_teon::{teon, Value};
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;

pub async fn delete_many(req_ctx: &request::Ctx) -> crate::path::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = DELETE | MANY | ENTRY;
    let (objects, count) = req_ctx.transaction_ctx().run_transaction(vec![model], |ctx: transaction::Ctx| async move {
        let objects = ctx.find_many_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await?;
        let mut count = 0;
        let mut ret_data: Vec<Value> = vec![];
        for (index, object) in objects.iter().enumerate() {
            object.delete_internal(path!["data", index]).await?;
            ret_data.push(object.to_json_internal(&path!["data", index]).await?);
            count += 1;
        }
        Ok((ret_data, count))
    }).await?;
    Ok(Response::data_meta(Value::Array(objects), teon!({"count": count}))?)
}
