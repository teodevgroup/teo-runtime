use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Object {
    inner: Arc<ObjectInner>
}

impl Object {

    pub fn new(struct_id: usize, fields: HashMap<String, crate::object::Object>) -> Self {
        Self {
            inner: Arc::new(ObjectInner {
                struct_id,
                fields: Arc::new(Mutex::new(fields)),
            })
        }
    }
}

#[derive(Debug)]
struct ObjectInner {
    struct_id: usize,
    fields: Arc<Mutex<HashMap<String, crate::object::Object>>>
}

