use teo_teon::Value;
use teo_result::Error;
use crate::object::Object;

impl<'a> TryFrom<&'a Object> for &'a Vec<Value> {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.as_array() {
            Some(v) => Ok(v),
            None => Err(Error::new(format!("object cannot convert to Vec<Value>: {:?}", value)))
        }
    }
}


impl<'a> TryFrom<&'a Object> for Vec<i32> {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not Vec<&str>: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for Vec<&'a str> {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not Vec<&str>: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for Vec<String> {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not Vec<String>: {:?}", value)))
        }
    }
}