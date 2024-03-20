use std::sync::Arc;
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<Vec<Value>> for Object {

    fn from(value: Vec<Value>) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Array(value)))
        }
    }
}

impl From<Vec<Object>> for Object {
    fn from(value: Vec<Object>) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Array(value))
        }
    }
}
