use teo_teon::Value;
use crate::object::Object;
use teo_result::Error;

impl<'a> TryFrom<&'a Object> for i32 {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not i32: {:?}", value)))
        }
    }
}