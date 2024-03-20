use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::model::relation::update::Update;
use crate::value::Value;

impl TryFrom<&Value> for Update {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;
        match enum_variant.value.as_str() {
            "noAction" => Ok(Update::NoAction),
            "nullify" => Ok(Update::Nullify),
            "update" => Ok(Update::Update),
            "delete" => Ok(Update::Delete),
            "deny" => Ok(Update::Deny),
            "default" => Ok(Update::Default),
            _ => Err(Error::new(format!("invalid update name: {:?}", value)))
        }
    }
}