use std::sync::Arc;
use crate::object::{Object, ObjectInner};
use crate::pipeline::pipeline::Pipeline;

impl From<Pipeline> for Object {

    fn from(value: Pipeline) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Pipeline(value)),
        }
    }
}