use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::config::client::ClientLanguage;
use teo_result::Error;
use crate::object::Object;

impl TryFrom<Object> for ClientLanguage {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        match enum_variant.value.as_str().unwrap() {
            "javaScript" => Ok(ClientLanguage::JavaScript),
            "typeScript" => Ok(ClientLanguage::TypeScript),
            "swift" => Ok(ClientLanguage::Swift),
            "kotlin" => Ok(ClientLanguage::Kotlin),
            "cSharp" => Ok(ClientLanguage::CSharp),
            "dart" => Ok(ClientLanguage::Dart),
            _ => Err(Error::new(format!("invalid client language name: {:?}", value)))
        }
    }
}