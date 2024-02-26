use indexmap::indexmap;
use key_path::KeyPath;
use teo_result::Error;

pub trait ErrorRuntimeExt {
    fn value_error(path: KeyPath, message: impl Into<String>) -> Error;
    fn value_error_message_only(message: impl Into<String>) -> Error;
    fn unique_error(path: KeyPath, constraint: impl AsRef<str>) -> Error;
    fn internal_server_error(path: KeyPath, message: impl Into<String>) -> Error;
    fn internal_server_error_message_only(message: impl Into<String>) -> Error;
    fn not_found(path: KeyPath) -> Error;
    fn not_found_message_only() -> Error;
    fn unauthorized_error(path: KeyPath, message: impl Into<String>) -> Error;
    fn unauthorized_error_message_only(message: impl Into<String>) -> Error;
}

impl ErrorRuntimeExt for Error {

    fn value_error(path: KeyPath, message: impl Into<String>) -> Error {
        Error::new_with_code_title_errors(
            "value is invalid",
            400,
            "ValueError",
            indexmap!{
            path.to_string() => message.into()
        }
        )
    }

    fn value_error_message_only(message: impl Into<String>) -> Error {
        Error::new_with_code_title(
            message.into(),
            400,
            "ValueError"
        )
    }

    fn unique_error(path: KeyPath, constraint: impl AsRef<str>) -> Error {
        Error::new_with_code_title_errors(
            "value is not unique",
            400,
            "UniqueError",
            indexmap! {
            path.to_string() => format!("value violates '{}' constraint", constraint.as_ref())
        }
        )
    }

    fn internal_server_error(path: KeyPath, message: impl Into<String>) -> Error {
        Error::new_with_code_title_errors(
            "internal server error",
            500,
            "InternalServerError",
            indexmap! {
            path.to_string() => message.into()
        }
        )
    }

    fn internal_server_error_message_only(message: impl Into<String>) -> Error {
        Error::new_with_code_title(
            message,
            500,
            "InternalServerError"
        )
    }

    fn not_found(path: KeyPath) -> Error {
        Error::new_with_code_title_errors(
            "not found",
            404,
            "NotFound",
            indexmap!{
            path.to_string() => "not found".to_owned()
        }
        )
    }

    fn not_found_message_only() -> Error {
        Error::new_with_code_title(
            "not found",
            404,
            "NotFound"
        )
    }

    fn unauthorized_error(path: KeyPath, message: impl Into<String>) -> Error {
        Error::new_with_code_title_errors(
            "unauthorized",
            401,
            "Unauthorized",
            indexmap! {
                path.to_string() => message.into()
            }
        )
    }

    fn unauthorized_error_message_only(message: impl Into<String>) -> Error {
        Error::new_with_code_title(
            message,
            401,
            "Unauthorized",
        )
    }
}
