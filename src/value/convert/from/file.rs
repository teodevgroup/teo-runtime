use crate::value::file::File;
use crate::value::Value;

impl From<File> for Value {

    fn from(value: File) -> Self {
        Self::File(value)
    }
}

impl From<&File> for Value {

    fn from(value: &File) -> Self {
        Self::File(value.clone())
    }
}