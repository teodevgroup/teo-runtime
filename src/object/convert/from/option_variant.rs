use std::sync::Arc;
use teo_teon::types::option_variant::OptionVariant;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<OptionVariant> for Object {

    fn from(value: OptionVariant) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::OptionVariant(value)))
        }
    }
}