use std::sync::Arc;
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<Value> for Object {

    fn from(value: Value) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(value)),
        }
    }
}

impl From<&Value> for Object {

    fn from(value: &Value) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(value.clone())),
        }
    }
}