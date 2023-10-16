use crate::r#struct;
use std::sync::Arc;
use crate::object::{Object, ObjectInner};

impl From<r#struct::Object> for Object {

    fn from(value: r#struct::Object) -> Self {
        Object {
            inner: Arc::new(ObjectInner::StructObject(value)),
        }
    }
}

impl From<&r#struct::Object> for Object {

    fn from(value: &r#struct::Object) -> Self {
        Object {
            inner: Arc::new(ObjectInner::StructObject(value.clone())),
        }
    }
}