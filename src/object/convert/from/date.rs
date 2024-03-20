use std::sync::Arc;
use chrono::NaiveDate;
use crate::value::Value;
use crate::object::{Object, ObjectInner};

impl From<NaiveDate> for Object {

    fn from(value: NaiveDate) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Date(value)))
        }
    }
}