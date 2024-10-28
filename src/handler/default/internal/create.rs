use key_path::{KeyPath, path};
use crate::value::Value;
use crate::teon;
use crate::action::Action;
use crate::connection::transaction;
use crate::model::Model;
use crate::error_ext;
use teo_result::Result;
use crate::request::Request;

pub(in crate::handler) async fn create_internal<'a>(transaction_ctx: transaction::Ctx, request: Request, create: Option<&'a Value>, include: Option<&'a Value>, select: Option<&'a Value>, model: &'static Model, path: &'a KeyPath, action: Action) -> Result<Value> {
    let obj = transaction_ctx.new_object(model, action, Some(request))?;
    match create {
        Some(create) => {
            if !create.is_dictionary() {
                return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "expect object"));
            }
            obj.set_teon_with_path(create, path).await
        }
        None => {
            obj.set_teon_with_path(&teon!({}), path).await
        }
    }?;
    obj.save_with_session_and_path(path).await?;
    let refreshed = obj.refreshed(include, select).await?;
    refreshed.to_teon_internal(&path!["data"]).await
}