use key_path::KeyPath;
use teo_result::Error;
use crate::model::Model;


pub fn unexpected_input(path: KeyPath) -> Error {
    Error::invalid_request_pathed(path, "unexpected input")
}

pub fn unexpected_input_value_with_reason(path: KeyPath, reason: impl Into<String>) -> Error {
    Error::invalid_request_pathed(path, reason)
}

pub fn missing_required_input(path: KeyPath) -> Error {
    Error::invalid_request_pathed(path, "missing required input")
}

pub fn missing_required_input_with_type(path: KeyPath, key: impl AsRef<str>) -> Error {
    Error::invalid_request_pathed(path, format!("missing required input {}", key.as_ref()))
}

pub fn cannot_disconnect_previous_relation(path: KeyPath) -> Error {
    Error::invalid_request_pathed(path, "cannot disconnect previous relation")
}

pub fn invalid_key_on_model(path: KeyPath, key: impl AsRef<str>, model: &Model) -> Error {
    Error::invalid_request_pathed(path, format!("key '{}' is invalid on model {}", key.as_ref(), model.path().join(".")))
}

pub fn deletion_denied(path: KeyPath, relation_name: &str) -> Error {
    Error::invalid_request_pathed(path, format!("deletion denied {}", relation_name))
}

pub fn updation_denied(path: KeyPath, relation_name: &str) -> Error {
    Error::invalid_request_pathed(path, format!("updation denied {}", relation_name))
}

pub fn invalid_operation(path: KeyPath, reason: impl AsRef<str>) -> Error {
    Error::internal_server_error_pathed(path, reason.as_ref())
}

pub fn unknown_database_write_error(path: KeyPath, reason: impl AsRef<str>) -> Error {
    Error::internal_server_error_pathed(path, format!("unknown database write error: {}", reason.as_ref()))
}

pub fn unknown_database_delete_error(path: KeyPath, reason: impl AsRef<str>) -> Error {
    Error::internal_server_error_pathed(path, format!("unknown database delete error: {}", reason.as_ref()))
}

pub fn unknown_database_find_error(path: KeyPath, reason: impl AsRef<str>) -> Error {
    Error::internal_server_error_pathed(path, format!("unknown database find error: {}", reason.as_ref()))
}

pub fn unique_value_duplicated(path: KeyPath, field: impl AsRef<str>) -> Error {
    Error::invalid_request_pathed(path, format!("unique value duplicated: {}", field.as_ref()))
}

pub fn invalid_sql_query(reason: impl AsRef<str>) -> Error {
    Error::internal_server_error_message(reason.as_ref())
}

pub fn object_is_not_saved_thus_cant_be_deleted(path: KeyPath) -> Error {
    Error::invalid_request_pathed(path, "object is not saved thus can't be deleted")
}

pub fn record_decoding_error(model: &str, path: impl AsRef<KeyPath>, t: &str) -> Error {
    Error::invalid_request_pathed(path.as_ref().clone(), format!("value decoding error on: {}: {}", model, t))
}