use serde_json::{Value as JsonValue};
use teo_teon::Value;
use crate::handler::Handler;

pub fn validate_and_transform_json_input_for_handler(handler: &Handler, json_body: &JsonValue) -> crate::path::Result<Value> {
    Ok(Value::Null)
}