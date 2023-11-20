use crate::object::Object;
use crate::r#struct;
use teo_result::Error;

impl<'a> TryFrom<&'a Object> for &'a r#struct::Object {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}

impl TryFrom<&Object> for r#struct::Object {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}

impl TryFrom<Object> for r#struct::Object {

    type Error = Error;

    fn try_from(value: Object) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}