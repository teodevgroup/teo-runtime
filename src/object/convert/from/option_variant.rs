use std::sync::Arc;
use crate::value::option_variant::OptionVariant;
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<OptionVariant> for Object {

    fn from(value: OptionVariant) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::OptionVariant(value)))
        }
    }
}