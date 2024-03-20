use crate::value::Value;
use teo_result::Error;
use crate::sort::Sort;

impl TryFrom<&Value> for Sort {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let enum_variant: &str = value.try_into()?;
        Ok(match enum_variant {
            "asc" => Sort::Asc,
            "desc" => Sort::Desc,
            _ => unreachable!(),
        })
    }
}

impl TryFrom<Value> for Sort {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let enum_variant: &str = value.try_into()?;
        Ok(match enum_variant {
            "asc" => Sort::Asc,
            "desc" => Sort::Desc,
            _ => unreachable!(),
        })
    }
}