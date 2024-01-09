use key_path::path;
use teo_teon::{teon, Value};
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::model::object::object::ErrorIfNotFound;

pub async fn copy_many(req_ctx: &request::Ctx) -> crate::path::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = COPY | MANY | ENTRY;
    let (retval, count) = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let objects = ctx.find_many_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await?;
        let copy = req_ctx.body().get("copy");
        let include = req_ctx.body().get("include");
        let select = req_ctx.body().get("select");
        let mut count = 0;
        let mut retval = vec![];
        for (index, object) in objects.iter().enumerate() {
            let value = object.copied_value();
            let new = ctx.new_object_with_teon_and_path(model, &teon!({}), &path![], action, Some(req_ctx.clone())).await?;
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
