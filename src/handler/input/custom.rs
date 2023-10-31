use serde_json::{Value as JsonValue};
use teo_teon::Value;
use crate::handler::Handler;
use crate::namespace::Namespace;

pub fn validate_and_transform_json_input_for_handler(handler: &Handler, json_body: &JsonValue, main_namespace: &Namespace) -> crate::path::Result<Value> {
    Ok(Value::Null)
}