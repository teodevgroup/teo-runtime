use crate::r#struct;
use teo_result::Error;
use crate::value::Value;

impl<'a> TryFrom<&'a Value> for &'a r#struct::Object {

    type Error = Error;

    fn try_from(value: &'a Value) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}

impl TryFrom<&Value> for r#struct::Object {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}

impl TryFrom<Value> for r#struct::Object {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}