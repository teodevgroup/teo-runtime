use teo_result::Error;
use crate::value::option_variant::OptionVariant;
use crate::value::Value;
use crate::object::Object;

impl<'a> TryFrom<&'a Object> for &'a OptionVariant {

    type Error = Error;

    fn try_from(value: &'a Object) -> Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not OptionVariant: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for OptionVariant {

    type Error = Error;

    fn try_from(value: &'a Object) -> Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not OptionVariant: {:?}", value)))
        }
    }
}