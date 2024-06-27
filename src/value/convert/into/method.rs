use teo_result::Error;
use crate::handler::Method;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl TryFrom<&Value> for Method {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        Ok(match interface_enum_variant.value.as_str() {
            "post" => Method::Post,
            "get" => Method::Get,
            "patch" => Method::Patch,
            "put" => Method::Put,
            "delete" => Method::Delete,
            _ => unreachable!(),
        })
    }
}
