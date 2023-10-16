use std::sync::Arc;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<Vec<Value>> for Object {

    fn from(value: Vec<Value>) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Array(value)))
        }
    }
}