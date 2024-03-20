use teo_result::Error;
use crate::handler::handler::Method;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::object::Object;

impl TryFrom<&Object> for Method {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;
        Ok(match enum_variant.value.as_str() {
            "post" => Method::Post,
            "get" => Method::Get,
            "patch" => Method::Patch,
            "put" => Method::Put,
            "delete" => Method::Delete,
            _ => unreachable!(),
        })
    }
}
