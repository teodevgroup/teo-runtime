use crate::value::Value;
use teo_result::Error;
use crate::object::{Object, ObjectInner};

impl TryFrom<Object> for Value {

    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.inner.as_ref() {
            ObjectInner::Teon(v) => Ok(v.clone()),
            _ => Err(Error::new(format!("object is not Teon: {:?}", value))),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Value {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_teon() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not teon: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for Value {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_teon() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not teon: {:?}", value)))
        }
    }
}