use crate::value::range::Range;
use crate::value::Value;
use crate::object::Object;
use teo_result::Error;

impl<'a> TryFrom<&'a Object> for &'a Range {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not Range: {:?}", value)))
        }
    }
}