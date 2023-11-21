use std::sync::Arc;
use crate::interface_enum_variant::InterfaceEnumVariant;
use crate::object::{Object, ObjectInner};

impl From<InterfaceEnumVariant> for Object {

    fn from(value: InterfaceEnumVariant) -> Self {
        Object {
            inner: Arc::new(ObjectInner::InterfaceEnumVariant(value))
        }
    }
}