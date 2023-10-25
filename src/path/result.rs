use key_path::KeyPath;
use crate::path::error::IntoPathedValueError;

pub type Result<T> = std::result::Result<T, crate::path::Error>;

pub trait IntoPathedValueResult<T> {
    fn into_pathed_value_result(self, path: KeyPath) -> Result<T>;
}

impl<T> IntoPathedValueResult<T> for teo_result::Result<T> {

    fn into_pathed_value_result(self, path: KeyPath) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into_pathed_value_error(path)),
        }
    }
}