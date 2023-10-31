use key_path::path;
use crate::action::Action;
use crate::action::action::*;
use crate::model::Model;
use serde_json::{Value as JsonValue};
use teo_teon::Value;
use crate::coder::json_to_teon;
use crate::namespace::Namespace;

pub fn validate_and_transform_json_input_for_builtin_action(model: &Model, action: Action, json_body: &JsonValue, main_namespace: &Namespace) -> crate::path::Result<Value> {
    let input= match action {
        FIND_UNIQUE_HANDLER => model.cache.shape.map.get("FindUniqueArgs").unwrap(),
        FIND_FIRST_HANDLER => model.cache.shape.map.get("FindFirstArgs").unwrap(),
        FIND_MANY_HANDLER => model.cache.shape.map.get("FindManyArgs").unwrap(),
        CREATE_HANDLER => model.cache.shape.map.get("CreateArgs").unwrap(),
        UPDATE_HANDLER => model.cache.shape.map.get("UpdateArgs").unwrap(),
        COPY_HANDLER => model.cache.shape.map.get("CopyArgs").unwrap(),
        UPSERT_HANDLER => model.cache.shape.map.get("UpsertArgs").unwrap(),
        DELETE_HANDLER => model.cache.shape.map.get("DeleteArgs").unwrap(),
        CREATE_MANY_HANDLER => model.cache.shape.map.get("CreateManyArgs").unwrap(),
        UPDATE_MANY_HANDLER => model.cache.shape.map.get("UpdateManyArgs").unwrap(),
        COPY_MANY_HANDLER => model.cache.shape.map.get("CopyManyArgs").unwrap(),
        DELETE_MANY_HANDLER => model.cache.shape.map.get("DeleteManyArgs").unwrap(),
        COUNT_HANDLER => model.cache.shape.map.get("CountArgs").unwrap(),
        AGGREGATE_HANDLER => model.cache.shape.map.get("AggregateArgs").unwrap(),
        GROUP_BY_HANDLER => model.cache.shape.map.get("GroupByArgs").unwrap(),
        _ => Err(crate::path::Error::value_error(path![], "unfound input definition"))?,
    };
    json_to_teon(json_body, &path![], input, main_namespace)
}