use crate::config::entity::Runtime;
use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl TryFrom<Value> for Runtime {

    type Error = Error;

    fn try_from(ref value: Value) -> Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        match interface_enum_variant.value.as_str() {
            "rust" => Ok(Runtime::Rust),
            "node" => Ok(Runtime::Node),
            "python" => Ok(Runtime::Python),
            _ => Err(Error::new(format!("invalid runtime name: {:?}", value)))
        }
    }
}