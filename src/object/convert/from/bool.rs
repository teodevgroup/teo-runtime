use std::sync::Arc;
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<bool> for Object {

    fn from(value: bool) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Bool(value)))
        }
    }
}