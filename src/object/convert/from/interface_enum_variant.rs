use std::sync::Arc;
use crate::object::{Object, ObjectInner};
use crate::value::interface_enum_variant::InterfaceEnumVariant;

impl From<InterfaceEnumVariant> for Object {

    fn from(value: InterfaceEnumVariant) -> Self {
        Object {
            inner: Arc::new(ObjectInner::InterfaceEnumVariant(value))
        }
    }
}