use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<DateTime<Utc>> for Object {

    fn from(value: DateTime<Utc>) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::DateTime(value)))
        }
    }
}