use std::sync::Arc;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<EnumVariant> for Object {

    fn from(value: EnumVariant) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::EnumVariant(value)))
        }
    }
}