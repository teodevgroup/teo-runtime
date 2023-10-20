use teo_result::Error;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::object::Object;

impl<'a> TryFrom<&'a Object> for &'a EnumVariant {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not EnumVariant: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for EnumVariant {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not EnumVariant: {:?}", value)))
        }
    }
}