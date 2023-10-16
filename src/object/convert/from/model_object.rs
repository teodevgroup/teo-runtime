use crate::model;
use std::sync::Arc;
use crate::object::{Object, ObjectInner};

impl From<model::Object> for Object {

    fn from(value: model::Object) -> Self {
        Object {
            inner: Arc::new(ObjectInner::ModelObject(value)),
        }
    }
}

impl From<&model::Object> for Object {

    fn from(value: &model::Object) -> Self {
        Object {
            inner: Arc::new(ObjectInner::ModelObject(value.clone())),
        }
    }
}