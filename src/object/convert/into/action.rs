use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use teo_result::Error;
use crate::action::Action;
use crate::object::Object;

impl TryFrom<Object> for Action {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        let int = enum_variant.value.to_int().unwrap();
        Ok(Action(int as u32))
    }
}

impl TryFrom<&Value> for Action {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let enum_variant: &EnumVariant = value.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("value is not enum variant: {:?}", value)))?
        }
        let int = enum_variant.value.to_int().unwrap();
        Ok(Action(int as u32))
    }
}