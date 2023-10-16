use std::sync::Arc;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<f64> for Object {

    fn from(value: f64) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Float(value)))
        }
    }
}