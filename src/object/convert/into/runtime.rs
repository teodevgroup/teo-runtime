use crate::config::entity::Runtime;
use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::object::Object;

impl TryFrom<Object> for Runtime {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;
        match enum_variant.value.as_str() {
            "rust" => Ok(Runtime::Rust),
            "node" => Ok(Runtime::Node),
            "python" => Ok(Runtime::Python),
            _ => Err(Error::new(format!("invalid runtime name: {:?}", value)))
        }
    }
}