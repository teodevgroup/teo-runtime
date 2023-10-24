use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use teo_result::Error;
use crate::object::Object;
use crate::sort::Sort;

impl TryFrom<&Object> for Sort {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        let val = enum_variant.value.as_str().unwrap();
        Ok(match val {
            "asc" => Sort::Asc,
            "desc" => Sort::Desc,
            _ => unreachable!(),
        })
    }
}

impl TryFrom<&Value> for Sort {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        let enum_variant: &EnumVariant = value.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("value is not enum variant: {:?}", value)))?
        }
        let val = enum_variant.value.as_str().unwrap();
        Ok(match val {
            "asc" => Sort::Asc,
            "desc" => Sort::Desc,
            _ => unreachable!(),
        })
    }
}