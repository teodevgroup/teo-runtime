use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use teo_result::Error;
use teo_teon::types::option_variant::OptionVariant;
use crate::action::Action;
use crate::object::Object;

impl TryFrom<Object> for Action {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: OptionVariant = teon.try_into()?;
        let int = enum_variant.value;
        Ok(Action(int as u32))
    }
}

impl TryFrom<&Value> for Action {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let enum_variant: &OptionVariant = value.try_into()?;
        let int = enum_variant.value;
        Ok(Action(int as u32))
    }
}