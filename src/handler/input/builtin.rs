use crate::action::Action;
use crate::model::Model;
use serde_json::{Value as JsonValue};
use teo_teon::Value;
use crate::namespace::Namespace;

pub fn validate_and_transform_json_input_for_builtin_action(model: &Model, action: Action, json_body: &JsonValue, main_namespace: &Namespace) -> crate::path::Result<Value> {
    Ok(Value::Null)
}