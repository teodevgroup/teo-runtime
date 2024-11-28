use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::handler::default::internal::update::update_internal;
use crate::request::Request;

pub async fn update_many(request: Request) -> teo_result::Result<Response> {
    let action = UPDATE | MANY | ENTRY;
    let (objects, count) = request.transaction_ctx().run_transaction(move |ctx: transaction::Ctx| {
        let request = request.clone();
        let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap().clone();
        async move {
            let input = request.body_value()?.as_dictionary().unwrap();
            let update = input.get("update");
            let include = input.get("include");
            let select = input.get("select");
            let objects = ctx.find_many_internal(&model, request.body_value()?, true, action, Some(request.clone()), path![]).await?;
            let mut count = 0;
            let mut ret_data: Vec<Value> = vec![];
            for (index, object) in objects.iter().enumerate() {
                let update_result = update_internal(object.clone(), update, include, select, &path!["update", index]).await?;
                ret_data.push(update_result);
                count += 1;
            }
            Ok((ret_data, count))
        }
    }).await?;
    Ok(Response::data_meta(Value::Array(objects), teon!({"count": count})))
}
