use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::create::create_internal;
use crate::object::error_ext;

pub async fn create_many(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = CREATE | MANY | ENTRY;
    let (objects, count) = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let input = req_ctx.body().as_dictionary().unwrap();
        let create = input.get("create");
        let include = input.get("include");
        let select = input.get("select");
        let create = create.unwrap().as_array().unwrap();
        let mut count = 0;
        let mut ret_data: Vec<Value> = vec![];
        for (index, val) in create.iter().enumerate() {
            let result = create_internal(ctx.clone(), req_ctx.clone(), Some(val), include, select, model, &path!["create", index], action).await?;
            count += 1;
            ret_data.push(result);
        }
        Ok((ret_data, count))
    }).await?;
    Ok(Response::data_meta(Value::Array(objects), teon!({"count": count})))
}
