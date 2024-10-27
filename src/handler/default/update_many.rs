use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::update::update_internal;

pub async fn update_many(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.request().handler_match().unwrap().path()).unwrap();
    let action = UPDATE | MANY | ENTRY;
    let (objects, count) = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let input = req_ctx.body().as_dictionary().unwrap();
        let update = input.get("update");
        let include = input.get("include");
        let select = input.get("select");
        let objects = ctx.find_many_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await?;
        let mut count = 0;
        let mut ret_data: Vec<Value> = vec![];
        for (index, object) in objects.iter().enumerate() {
            let update_result = update_internal(object.clone(), update, include, select, &path!["update", index]).await?;
            ret_data.push(update_result);
            count += 1;
        }
        Ok((ret_data, count))
    }).await?;
    Ok(Response::data_meta(Value::Array(objects), teon!({"count": count})))
}
