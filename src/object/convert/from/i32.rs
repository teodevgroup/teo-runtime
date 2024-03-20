use std::sync::Arc;
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<i32> for Object {

    fn from(value: i32) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Int(value)))
        }
    }
}