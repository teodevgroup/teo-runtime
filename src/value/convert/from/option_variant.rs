use crate::value::option_variant::OptionVariant;
use crate::value::Value;

impl From<OptionVariant> for Value {

    fn from(value: OptionVariant) -> Self {
        Self::OptionVariant(value)
    }
}

impl From<&OptionVariant> for Value {

    fn from(value: &OptionVariant) -> Self {
        Self::OptionVariant(value.clone())
    }
}