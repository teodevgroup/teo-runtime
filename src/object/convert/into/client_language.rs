use crate::config::client::{ClientLanguage, TypeScriptHTTPProvider};
use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::object::Object;

impl TryFrom<Object> for ClientLanguage {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;

        match enum_variant.value.as_str() {
            "typeScript" => {
                let http_provider: TypeScriptHTTPProvider = enum_variant.args().unwrap().get_optional("httpProvider")?.unwrap_or(TypeScriptHTTPProvider::Fetch);
                Ok(ClientLanguage::TypeScript(http_provider))
            },
            "swift" => Ok(ClientLanguage::Swift),
            "kotlin" => Ok(ClientLanguage::Kotlin),
            "cSharp" => Ok(ClientLanguage::CSharp),
            "dart" => Ok(ClientLanguage::Dart),
            _ => Err(Error::new(format!("invalid client language name: {:?}", value)))
        }
    }
}