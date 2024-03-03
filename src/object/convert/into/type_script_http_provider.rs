use crate::config::client::TypeScriptHTTPProvider;
use teo_result::Error;
use crate::interface_enum_variant::InterfaceEnumVariant;
use crate::object::Object;

impl TryFrom<Object> for TypeScriptHTTPProvider {

    type Error = Error;

    fn try_from(ref value: Object) -> Result<Self, Self::Error> {
        Self::try_from(value)
    }
}

impl TryFrom<&Object> for TypeScriptHTTPProvider {

    type Error = Error;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;
        match enum_variant.value.as_str() {
            "fetch" => Ok(TypeScriptHTTPProvider::Fetch),
            "taro" => Ok(TypeScriptHTTPProvider::Taro),
            "wechat" => Ok(TypeScriptHTTPProvider::WeChat),
            _ => Err(Error::new(format!("invalid TypeScriptHTTPProvider value: {:?}", value)))
        }
    }
}