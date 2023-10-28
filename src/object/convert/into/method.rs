use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use teo_result::Error;
use crate::handler::handler::Method;
use crate::object::Object;

impl TryFrom<&Object> for Method {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        let val = enum_variant.value.as_str().unwrap();
        Ok(match val {
            "post" => Method::Post,
            "get" => Method::Get,
            "patch" => Method::Patch,
            "put" => Method::Put,
            "delete" => Method::Delete,
            _ => unreachable!(),
        })
    }
}

impl TryFrom<&Value> for Method {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let enum_variant: &EnumVariant = value.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("value is not enum variant: {:?}", value)))?
        }
        let val = enum_variant.value.as_str().unwrap();
        Ok(match val {
            "post" => Method::Post,
            "get" => Method::Get,
            "patch" => Method::Patch,
            "put" => Method::Put,
            "delete" => Method::Delete,
            _ => unreachable!(),
        })
    }
}