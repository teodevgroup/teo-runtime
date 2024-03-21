use teo_parser::r#type::Type;
use teo_result::Error;
use crate::value::Value;

impl<'a> TryFrom<&'a Value> for &'a Type {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value.as_type() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not type: {:?}", value)))
        }
    }
}

impl TryFrom<Value> for Type {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Type(o) => Ok(o),
            _ => Err(Error::new(format!("object is not type: {:?}", value)))
        }
    }
}