use teo_teon::Value;
use teo_result::Error;
use crate::object::Object;

impl TryFrom<Object> for String {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not String: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for String {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not String: {:?}", value)))
        }
    }
}

impl TryFrom<Object> for Option<String> {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not Option<String>: {:?}", value)))
        }
    }
}