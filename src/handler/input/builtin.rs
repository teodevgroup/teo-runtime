use key_path::path;
use crate::action::Action;
use crate::action::action::*;
use crate::model::Model;
use serde_json::{Value as JsonValue};
use teo_parser::r#type::synthesized_shape_reference::SynthesizedShapeReferenceKind;
use crate::value::Value;
use crate::coder::json_to_teon;
use crate::namespace::Namespace;


pub fn validate_and_transform_json_input_for_builtin_action(model: &Model, action: Action, json_body: &JsonValue, main_namespace: &Namespace) -> teo_result::Result<Value> {
    let input= match action {
        FIND_UNIQUE_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::FindUniqueArgs).unwrap(),
        FIND_FIRST_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::FindFirstArgs).unwrap(),
        FIND_MANY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::FindManyArgs).unwrap(),
        CREATE_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::CreateArgs).unwrap(),
        UPDATE_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::UpdateArgs).unwrap(),
        COPY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::CopyArgs).unwrap(),
        UPSERT_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::UpsertArgs).unwrap(),
        DELETE_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::DeleteArgs).unwrap(),
        CREATE_MANY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::CreateManyArgs).unwrap(),
        UPDATE_MANY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::UpdateManyArgs).unwrap(),
        COPY_MANY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::CopyManyArgs).unwrap(),
        DELETE_MANY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::DeleteManyArgs).unwrap(),
        COUNT_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::CountArgs).unwrap(),
        AGGREGATE_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::AggregateArgs).unwrap(),
        GROUP_BY_HANDLER => model.cache().shape.get(SynthesizedShapeReferenceKind::GroupByArgs).unwrap(),
        _ => Err(teo_result::Error::invalid_request_pathed(path![], "unfound input definition"))?,
    };
    json_to_teon(json_body, &path![], input, main_namespace)
}