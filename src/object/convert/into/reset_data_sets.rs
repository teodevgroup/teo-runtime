use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::config::test::ResetDataSets;
use teo_result::Error;
use crate::object::Object;

impl TryFrom<Object> for ResetDataSets {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        match enum_variant.value.as_str().unwrap() {
            "auto" => Ok(ResetDataSets::Auto),
            "dataSets" => {
                let map = enum_variant.args.unwrap();
                let names = map.get("names").unwrap();
                Ok(ResetDataSets::DataSets(names.as_array().unwrap().iter().map(|item| item.as_array().unwrap().iter().map(|part| part.as_str().unwrap().to_owned()).collect()).collect()))
            },
            _ => Err(Error::new(format!("invalid reset data sets name: {:?}", value)))
        }
    }
}