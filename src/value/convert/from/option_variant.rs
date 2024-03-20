use crate::value::option_variant::OptionVariant;
use crate::value::Value;
use teo_parser::value::option_variant::OptionVariant as ParserOptionVariant;

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

impl From<ParserOptionVariant> for OptionVariant {
    fn from(value: ParserOptionVariant) -> Self {
        Self {
            value: value.value,
            display: value.display,
        }
    }
}