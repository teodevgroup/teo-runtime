use teo_teon::Value;
use crate::error::Error;
use crate::object::Object;

impl TryFrom<Object> for bool {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not bool: {:?}", value)))
        }
    }
}