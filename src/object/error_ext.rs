use key_path::KeyPath;
use crate::model::Model;

pub fn unexpected_input(path: KeyPath) -> crate::path::Error {
    crate::path::Error::value_error(path, "unexpected input")
}

pub fn unexpected_input_value_with_reason(path: KeyPath, reason: impl Into<String>) -> crate::path::Error {
    crate::path::Error::value_error(path, reason)
}

pub fn missing_required_input(path: KeyPath) -> crate::path::Error {
    crate::path::Error::value_error(path, "missing required input")
}

pub fn missing_required_input_with_type(path: KeyPath, key: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::value_error(path, format!("missing required input {}", key.as_ref()))
}

pub fn cannot_disconnect_previous_relation(path: KeyPath) -> crate::path::Error {
    crate::path::Error::value_error(path, "cannot disconnect previous relation")
}

pub fn invalid_key_on_model(path: KeyPath, key: impl AsRef<str>, model: &Model) -> crate::path::Error {
    crate::path::Error::value_error(path, format!("key '{}' is invalid on model {}", key.as_ref(), model.path.join(".")))
}

pub fn deletion_denied(path: KeyPath, relation_name: &str) -> crate::path::Error {
    crate::path::Error::value_error(path, format!("deletion denied {}", relation_name))
}

pub fn updation_denied(path: KeyPath, relation_name: &str) -> crate::path::Error {
    crate::path::Error::value_error(path, format!("updation denied {}", relation_name))
}

pub fn invalid_operation(path: KeyPath, reason: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::internal_server_error(path, reason.as_ref().to_string())
}

pub fn not_found(path: KeyPath) -> crate::path::Error {
    crate::path::Error::not_found(path)
}

pub fn unknown_database_write_error(path: KeyPath, reason: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::internal_server_error(path, format!("unknown database write error: {}", reason.as_ref()))
}

pub fn unknown_database_delete_error(path: KeyPath, reason: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::internal_server_error(path, format!("unknown database delete error: {}", reason.as_ref()))
}

pub fn unknown_database_find_error(path: KeyPath, reason: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::internal_server_error(path, format!("unknown database find error: {}", reason.as_ref()))
}

pub fn unique_value_duplicated(path: KeyPath, field: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::value_error(path, format!("unique value duplicated: {}", field.as_ref()))
}

pub fn invalid_sql_query(reason: impl AsRef<str>) -> crate::path::Error {
    crate::path::Error::internal_server_error_message_only(reason.as_ref())
}

pub fn object_is_not_saved_thus_cant_be_deleted(path: KeyPath) -> crate::path::Error {
    crate::path::Error::value_error(path, "object is not saved thus can't be deleted")
}

pub fn record_decoding_error(model: &str, path: impl AsRef<KeyPath>, t: &str) -> crate::path::Error {
    crate::path::Error::value_error(path.as_ref().clone(), format!("value decoding error on: {}: {}", model, t))
}