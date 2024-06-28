use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use indexmap::{IndexMap};
use maplit::btreemap;
use teo_parser::r#type::synthesized_shape::SynthesizedShape;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::interface::{Field, Interface, interface};
use crate::Value;

#[derive(Debug, Clone)]
pub struct Builder {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    path: Vec<String>,
    parser_path: Vec<usize>,
    comment: Option<Comment>,
    fields: IndexMap<String, Field>,
    generic_names: Vec<String>,
    extends: Vec<Type>,
    shape: SynthesizedShape,
    generate_client: AtomicBool,
    generate_entity: AtomicBool,
    data: Arc<Mutex<BTreeMap<String, Value>>>
}

impl Builder {
    pub fn new(path: Vec<String>, parser_path: Vec<usize>, comment: Option<Comment>, fields: IndexMap<String, Field>, generic_names: Vec<String>, extends: Vec<Type>, shape: SynthesizedShape) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                parser_path,
                comment,
                fields,
                generic_names,
                extends,
                shape,
                generate_client: AtomicBool::new(true),
                generate_entity: AtomicBool::new(true),
                data: Arc::new(Mutex::new(btreemap! {})),
            })
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn parser_path(&self) -> &Vec<usize> {
        &self.inner.parser_path
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    pub fn fields(&self) -> &IndexMap<String, Field> {
        &self.inner.fields
    }

    pub fn generic_names(&self) -> &Vec<String> {
        &self.inner.generic_names
    }

    pub fn extends(&self) -> &Vec<Type> {
        &self.inner.extends
    }

    pub fn shape(&self) -> &SynthesizedShape {
        &self.inner.shape
    }

    pub fn generate_client(&self) -> bool {
        self.inner.generate_client.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_generate_client(&self, generate_client: bool) {
        self.inner.generate_client.store(generate_client, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn generate_entity(&self) -> bool {
        self.inner.generate_entity.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_generate_entity(&self, generate_entity: bool) {
        self.inner.generate_entity.store(generate_entity, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn data(&self) -> BTreeMap<String, Value> {
        self.inner.data.lock().unwrap().clone()
    }

    pub fn set_data(&self, data: BTreeMap<String, Value>) {
        *self.inner.data.lock().unwrap() = data;
    }

    pub fn insert_data_entry(&self, key: String, value: Value) {
        self.inner.data.lock().unwrap().insert(key, value);
    }

    pub fn remove_data_entry(&self, key: &str) {
        self.inner.data.lock().unwrap().remove(key);
    }

    pub fn data_entry(&self, key: &str) -> Option<Value> {
        self.inner.data.lock().unwrap().get(key).cloned()
    }

    pub(crate) fn build(self) -> Interface {
        Interface {
            inner: Arc::new(interface::Inner {
                path: self.inner.path.clone(),
                parser_path: self.inner.parser_path.clone(),
                comment: self.inner.comment.clone(),
                fields: self.inner.fields.clone(),
                generic_names: self.inner.generic_names.clone(),
                extends: self.inner.extends.clone(),
                shape: self.inner.shape.clone(),
                generate_client: self.inner.generate_client.load(std::sync::atomic::Ordering::Relaxed),
                generate_entity: self.inner.generate_entity.load(std::sync::atomic::Ordering::Relaxed),
                data: self.inner.data.lock().unwrap().clone(),
            })
        }
    }
}