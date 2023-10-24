use std::fmt::{Display, Formatter};
use indexmap::{IndexMap, indexmap};
use key_path::KeyPath;

#[derive(Debug)]
pub struct Error {
    pub title: &'static str,
    pub message: String,
    pub fields: Option<IndexMap<String, String>>,
    pub code: i32,
}

impl Error {

    pub fn value_error(path: KeyPath, message: impl Into<String>) -> Self {
        Self {
            title: "ValueError",
            message: "value is invalid".to_owned(),
            fields: Some(indexmap!{
                path.to_string() => message.into()
            }),
            code: 400
        }
    }

    pub fn unique_error(path: KeyPath, constraint: String) -> Self {
        Self {
            title: "UniqueError",
            message: "value is not unique".to_owned(),
            fields: Some(indexmap! {
                path.to_string() => format!("value violates '{}' constraint", constraint)
            }),
            code: 400
        }
    }

    pub fn internal_server_error(path: KeyPath, message: String) -> Self {
        Self {
            title: "InternalServerError",
            message: "internal server error".to_owned(),
            fields: Some(indexmap! {
                path.to_string() => message
            }),
            code: 500
        }
    }

    pub fn internal_server_error_message_only(message: String) -> Self {
        Self {
            title: "InternalServerError",
            message,
            fields: None,
            code: 500,
        }
    }

    pub fn unauthorized_error(path: KeyPath, message: String) -> Self {
        Self {
            title: "Unauthorized",
            message: "unauthorized".to_owned(),
            fields: Some(indexmap! {
                path.to_string() => message
            }),
            code: 401
        }
    }
}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.title)?;
        f.write_str(":")?;
        f.write_str(&format!("{}", self.code))?;
        f.write_str("(")?;
        f.write_str(&self.message)?;
        f.write_str(")")?;
        if let Some(fields) = &self.fields {
            f.write_str("[")?;
            fields.iter().for_each(|(k, v)| {
                f.write_str(k)?;
                f.write_str(": ")?;
                f.write_str(v)?;
            })?;
            f.write_str("]")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error { }

impl From<teo_result::Error> for Error {

    fn from(value: teo_result::Error) -> Self {
        Self::internal_server_error_message_only(value.message)
    }
}

impl From<Error> for teo_result::Error {

    fn from(value: Error) -> Self {
        teo_result::Error::new(value.message)
    }
}