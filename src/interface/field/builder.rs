use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::interface::Field;
use crate::optionality::Optionality;
use crate::Value;

pub struct Builder {
    inner: Arc<Inner>,
}

struct Inner {
    name: String,
    comment: Option<Comment>,
    r#type: Type,
    optionality: Arc<Mutex<Optionality>>,
    data: Arc<Mutex<BTreeMap<String, Value>>>
}

impl Builder {
    pub fn new(name: String, comment: Option<Comment>, r#type: Type) -> Self {
        Self {
            inner: Arc::new(Inner {
                name,
                comment,
                r#type,
                optionality: Arc::new(Mutex::new(Optionality::Required)),
                data: Arc::new(Mutex::new(BTreeMap::new())),
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    pub fn r#type(&self) -> &Type {
        &self.inner.r#type
    }

    pub fn optionality(&self) -> Optionality {
        self.inner.optionality.lock().unwrap().clone()
    }

    pub fn set_optionality(&self, optionality: Optionality) {
        *self.inner.optionality.lock().unwrap() = optionality;
    }

    pub fn data(&self) -> BTreeMap<String, Value> {
        self.inner.data.lock().unwrap().clone()
    }

    pub fn insert_data_entry(&self, key: String, value: Value) {
        self.inner.data.lock().unwrap().insert(key, value);
    }

    pub fn remove_data_entry(&self, key: &str) {
        self.inner.data.lock().unwrap().remove(key);
    }

    pub fn set_data(&self, data: BTreeMap<String, Value>) {
        *self.inner.data.lock().unwrap() = data;
    }

    pub fn data_entry(&self, key: &str) -> Option<Value> {
        self.inner.data.lock().unwrap().get(key).cloned()
    }

    pub(crate) fn build(self) -> Field {
        Field {
            name: self.inner.name.clone(),
            comment: self.inner.comment.clone(),
            r#type: self.inner.r#type.clone(),
            optionality: self.optionality(),
            data: self.data(),
        }
    }
}