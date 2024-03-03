use crate::config::client::ClientHost;
use teo_result::Error;
use crate::interface_enum_variant::InterfaceEnumVariant;
use crate::object::Object;

impl TryFrom<Object> for ClientHost {

    type Error = Error;

    fn try_from(ref value: Object) -> Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;
        let string: String = enum_variant.args().unwrap().get("value")?;
        match enum_variant.value.as_str() {
            "string" => Ok(ClientHost::String(string)),
            "inject" => Ok(ClientHost::Inject(string)),
            _ => Err(Error::new(format!("invalid client host name: {:?}", value)))
        }
    }
}