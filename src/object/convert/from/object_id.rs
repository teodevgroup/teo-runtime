use std::sync::Arc;
use bson::oid::ObjectId;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<ObjectId> for Object {

    fn from(value: ObjectId) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::ObjectId(value)))
        }
    }
}