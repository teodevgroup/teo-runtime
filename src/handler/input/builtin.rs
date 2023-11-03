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
        FIND_UNIQUE_HANDLER => model.cache.shape.map.get(&("FindUniqueArgs".to_owned(), None)).unwrap(),
        FIND_FIRST_HANDLER => model.cache.shape.map.get(&("FindFirstArgs".to_owned(), None)).unwrap(),
        FIND_MANY_HANDLER => model.cache.shape.map.get(&("FindManyArgs".to_owned(), None)).unwrap(),
        CREATE_HANDLER => model.cache.shape.map.get(&("CreateArgs".to_owned(), None)).unwrap(),
        UPDATE_HANDLER => model.cache.shape.map.get(&("UpdateArgs".to_owned(), None)).unwrap(),
        COPY_HANDLER => model.cache.shape.map.get(&("CopyArgs".to_owned(), None)).unwrap(),
        UPSERT_HANDLER => model.cache.shape.map.get(&("UpsertArgs".to_owned(), None)).unwrap(),
        DELETE_HANDLER => model.cache.shape.map.get(&("DeleteArgs".to_owned(), None)).unwrap(),
        CREATE_MANY_HANDLER => model.cache.shape.map.get(&("CreateManyArgs".to_owned(), None)).unwrap(),
        UPDATE_MANY_HANDLER => model.cache.shape.map.get(&("UpdateManyArgs".to_owned(), None)).unwrap(),
        COPY_MANY_HANDLER => model.cache.shape.map.get(&("CopyManyArgs".to_owned(), None)).unwrap(),
        DELETE_MANY_HANDLER => model.cache.shape.map.get(&("DeleteManyArgs".to_owned(), None)).unwrap(),
        COUNT_HANDLER => model.cache.shape.map.get(&("CountArgs".to_owned(), None)).unwrap(),
        AGGREGATE_HANDLER => model.cache.shape.map.get(&("AggregateArgs".to_owned(), None)).unwrap(),
        GROUP_BY_HANDLER => model.cache.shape.map.get(&("GroupByArgs".to_owned(), None)).unwrap(),
        _ => Err(crate::path::Error::value_error(path![], "unfound input definition"))?,
    };
    json_to_teon(json_body, &path![], input, main_namespace)
}