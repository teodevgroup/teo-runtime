use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::config::entity::Runtime;
use teo_result::Error;
use crate::object::Object;

impl TryFrom<Object> for Runtime {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        match enum_variant.value.as_str() {
            "rust" => Ok(Runtime::Rust),
            "node" => Ok(Runtime::Node),
            "python" => Ok(Runtime::Python),
            _ => Err(Error::new(format!("invalid runtime name: {:?}", value)))
        }
    }
}