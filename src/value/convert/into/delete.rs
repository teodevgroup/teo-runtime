use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::model::relation::delete::Delete;
use crate::value::Value;

impl TryFrom<&Value> for Delete {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        match interface_enum_variant.value.as_str() {
            "noAction" => Ok(Delete::NoAction),
            "nullify" => Ok(Delete::Nullify),
            "cascade" => Ok(Delete::Cascade),
            "deny" => Ok(Delete::Deny),
            "default" => Ok(Delete::Default),
            _ => Err(Error::new(format!("invalid delete name: {:?}", value)))
        }
    }
}