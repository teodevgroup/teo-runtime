use teo_result::Error;
use crate::admin::language::Language;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl TryFrom<Value> for Language {

    type Error = Error;

    fn try_from(ref value: Value) -> Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        Self::from_str(interface_enum_variant.value())
    }
}