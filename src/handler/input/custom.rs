use key_path::path;
use serde_json::{Value as JsonValue};
use teo_teon::Value;
use crate::coder::json_to_teon::json_to_teon_with_type;
use crate::handler::Handler;
use crate::namespace::Namespace;

pub fn validate_and_transform_json_input_for_handler(handler: &Handler, json_body: &JsonValue, main_namespace: &Namespace) -> teo_result::Result<Value> {
    json_to_teon_with_type(json_body, &path![], &handler.input_type, main_namespace)
}