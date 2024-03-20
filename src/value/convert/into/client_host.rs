use crate::config::client::ClientHost;
use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl TryFrom<Value> for ClientHost {

    type Error = Error;

    fn try_from(ref value: Value) -> Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        let string: String = interface_enum_variant.args().unwrap().get("value")?;
        match interface_enum_variant.value.as_str() {
            "string" => Ok(ClientHost::String(string)),
            "inject" => Ok(ClientHost::Inject(string)),
            _ => Err(Error::new(format!("invalid client host name: {:?}", value)))
        }
    }
}