use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Object {
    inner: Arc<ObjectInner>
}

impl Object {

    pub fn new(struct_path: Vec<String>, fields: BTreeMap<String, crate::object::Object>) -> Self {
        Self {
            inner: Arc::new(ObjectInner {
                struct_path,
                fields: Mutex::new(fields),
            })
        }
    }
}

impl Serialize for Object {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}

#[derive(Debug)]
struct ObjectInner {
    struct_path: Vec<String>,
    fields: Mutex<BTreeMap<String, crate::object::Object>>
}

