use key_path::{KeyPath, path};
use teo_teon::{teon, Value};
use crate::model::Object;
use teo_result::Result;

pub(in crate::handler) async fn update_internal<'a>(object: Object, update: Option<&'a Value>, include: Option<&'a Value>, select: Option<&'a Value>, path: &'a KeyPath) -> Result<Value> {
    let empty = teon!({});
    let updater = if update.is_some() { update.unwrap() } else { &empty };
    object.set_teon_with_path(updater, &path).await?;
    object.save_with_session_and_path(path).await?;
    let refreshed = object.refreshed(include, select).await?;
    refreshed.to_teon_internal(&path!["data"]).await
}