use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::model::relation::delete::Delete;
use crate::model::relation::update::Update;
use crate::optionality::Optionality;
use crate::Value;

pub struct Builder {
    inner: Arc<Inner>
}

pub struct Inner {
    pub name: String,
    pub comment: Option<Comment>,
    pub r#type: Type,
    pub optionality: Arc<Mutex<Optionality>>,
    pub model: Arc<Mutex<Vec<String>>>,
    pub through: Arc<Mutex<Option<Vec<String>>>>,
    pub local: Arc<Mutex<Option<String>>>,
    pub foreign: Arc<Mutex<Option<String>>>,
    pub is_vec: AtomicBool,
    pub fields: Arc<Mutex<Vec<String>>>,
    pub references: Arc<Mutex<Vec<String>>>,
    pub delete: Arc<Mutex<Delete>>,
    pub update: Arc<Mutex<Update>>,
    pub has_foreign_key: AtomicBool,
    pub data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {
    pub fn new(name: String, comment: Option<Comment>, r#type: Type) -> Self {
        Self {
            inner: Arc::new(Inner {
                name,
                comment,
                r#type,
                optionality: Arc::new(Mutex::new(Optionality::Optional)),
                model: Arc::new(Mutex::new(vec![])),
                through: Arc::new(Mutex::new(None)),
                local: Arc::new(Mutex::new(None)),
                foreign: Arc::new(Mutex::new(None)),
                is_vec: AtomicBool::new(false),
                fields: Arc::new(Mutex::new(vec![])),
                references: Arc::new(Mutex::new(vec![])),
                delete: Arc::new(Mutex::new(Delete::Nullify)),
                update: Arc::new(Mutex::new(Update::Nullify)),
                has_foreign_key: AtomicBool::new(false),
                data: Arc::new(Mutex::new(BTreeMap::new())),
            })
        }
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_str()
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

    pub fn model(&self) -> Vec<String> {
        self.inner.model.lock().unwrap().clone()
    }

    pub fn set_model(&self, model: Vec<String>) {
        *self.inner.model.lock().unwrap() = model;
    }

    pub fn through(&self) -> Option<Vec<String>> {
        self.inner.through.lock().unwrap().clone()
    }

    pub fn set_through(&self, through: Option<Vec<String>>) {
        *self.inner.through.lock().unwrap() = through;
    }

    pub fn local(&self) -> Option<String> {
        self.inner.local.lock().unwrap().clone()
    }

    pub fn set_local(&self, local: Option<String>) {
        *self.inner.local.lock().unwrap() = local;
    }

    pub fn foreign(&self) -> Option<String> {
        self.inner.foreign.lock().unwrap().clone()
    }

    pub fn set_foreign(&self, foreign: Option<String>) {
        *self.inner.foreign.lock().unwrap() = foreign;
    }

    pub fn is_vec(&self) -> bool {
        self.inner.is_vec.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_is_vec(&self, is_vec: bool) {
        self.inner.is_vec.store(is_vec, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn fields(&self) -> Vec<String> {
        self.inner.fields.lock().unwrap().clone()
    }

    pub fn set_fields(&self, fields: Vec<String>) {
        *self.inner.fields.lock().unwrap() = fields;
    }

    pub fn references(&self) -> Vec<String> {
        self.inner.references.lock().unwrap().clone()
    }

    pub fn set_references(&self, references: Vec<String>) {
        *self.inner.references.lock().unwrap() = references;
    }

    pub fn delete(&self) -> Delete {
        *self.inner.delete.lock().unwrap()
    }

    pub fn set_delete(&self, delete: Delete) {
        *self.inner.delete.lock().unwrap() = delete;
    }

    pub fn update(&self) -> Update {
        *self.inner.update.lock().unwrap()
    }

    pub fn set_update(&self, update: Update) {
        *self.inner.update.lock().unwrap() = update;
    }

    pub fn has_foreign_key(&self) -> bool {
        self.inner.has_foreign_key.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_has_foreign_key(&self, has_foreign_key: bool) {
        self.inner.has_foreign_key.store(has_foreign_key, std::sync::atomic::Ordering::Relaxed);
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
}