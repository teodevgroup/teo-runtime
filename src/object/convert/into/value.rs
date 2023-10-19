use teo_teon::Value;
use teo_result::Error;
use crate::object::{Object, ObjectInner};

impl TryFrom<Object> for Value {

    type Error = Error;

    fn try_from(value: Object) -> std::result::Result<Self, Self::Error> {
        match value.inner.as_ref() {
            ObjectInner::Teon(v) => Ok(v.clone()),
            _ => Err(Error::new(format!("object is not Teon: {:?}", value))),
        }
    }
}