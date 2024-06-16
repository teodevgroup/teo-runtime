use teo_result::Error;
use hyper::Method;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl TryFrom<&Value> for Method {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        Ok(match interface_enum_variant.value.as_str() {
            "post" => Method::POST,
            "get" => Method::GET,
            "patch" => Method::PATCH,
            "put" => Method::PUT,
            "delete" => Method::DELETE,
            _ => unreachable!(),
        })
    }
}
