use key_path::path;
use crate::value::Value;
use crate::teon;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::request::Request;

pub async fn copy_many(request: &Request) -> teo_result::Result<Response> {
    let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap();
    let action = COPY | MANY | ENTRY;
    let (retval, count) = request.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let objects = ctx.find_many_internal(model, request.body_value()?, true, action, Some(request.clone()), path![]).await?;
        let copy = request.body_value()?.get("copy");
        let include = request.body_value()?.get("include");
        let select = request.body_value()?.get("select");
        let mut count = 0;
        let mut retval = vec![];
        for (index, object) in objects.iter().enumerate() {
            let value = object.copied_value();
            let new = ctx.new_object_with_teon_and_path(model, &teon!({}), &path![], action, Some(request.clone())).await?;
            new.update_teon(&value).await?;
            if let Some(copy) = copy {
                new.set_teon_with_path(copy, &path!["copy"]).await?;
            }
            new.save_with_session_and_path(&path!["copy"]).await?;
            let refreshed = new.refreshed(include, select).await?;
            retval.push(refreshed.to_teon_internal(&path!["data", index]).await?);
            count += 1;
        }
        Ok((retval, count))
    }).await?;
    Ok(Response::data_meta(Value::Array(retval), teon!({"count": count})))
}
