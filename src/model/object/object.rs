use std::sync::Arc;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Object {
    inner: Arc<ObjectInner>
}

impl Object {

}

#[derive(Debug)]
struct ObjectInner {

}

impl Serialize for Object {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}