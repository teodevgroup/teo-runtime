use crate::value::Value;
use teo_result::Error;
use crate::value::option_variant::OptionVariant;
use crate::action::Action;

impl TryFrom<Value> for Action {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Action::try_from(&value)

    }
}

impl TryFrom<&Value> for Action {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let option_variant: &OptionVariant = value.try_into()?;
        Ok(Action(option_variant.value as u32))
    }
}
