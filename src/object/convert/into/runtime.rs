use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::config::entity::Runtime;
use crate::error::Error;
use crate::object::Object;

impl TryFrom<Object> for Runtime {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        match enum_variant.value.as_str().unwrap() {
            "rust" => Ok(Runtime::Rust),
            "node" => Ok(Runtime::Node),
            "python" => Ok(Runtime::Python),
            _ => Err(Error::new(format!("invalid runtime name: {:?}", value)))
        }
    }
}