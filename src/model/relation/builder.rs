use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::model::{Field, Relation};
use crate::model::relation::delete::Delete;
use crate::model::relation::relation;
use crate::model::relation::update::Update;
use crate::optionality::Optionality;
use crate::traits::named::Named;
use crate::Value;

#[derive(Clone)]
pub struct Builder {
    inner: Arc<Inner>
}

pub struct Inner {
    name: String,
    comment: Option<Comment>,
    r#type: Type,
    optionality: Arc<Mutex<Optionality>>,
    model: Arc<Mutex<Vec<String>>>,
    through: Arc<Mutex<Option<Vec<String>>>>,
    local: Arc<Mutex<Option<String>>>,
    foreign: Arc<Mutex<Option<String>>>,
    is_vec: AtomicBool,
    fields: Arc<Mutex<Vec<String>>>,
    references: Arc<Mutex<Vec<String>>>,
    delete: Arc<Mutex<Delete>>,
    update: Arc<Mutex<Update>>,
    delete_specified: AtomicBool,
    update_specified: AtomicBool,
    data: Arc<Mutex<BTreeMap<String, Value>>>,
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
                delete_specified: AtomicBool::new(false),
                update_specified: AtomicBool::new(false),
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
        self.inner.delete_specified.store(true, Ordering::Relaxed);
    }

    pub fn update(&self) -> Update {
        *self.inner.update.lock().unwrap()
    }

    pub fn set_update(&self, update: Update) {
        *self.inner.update.lock().unwrap() = update;
        self.inner.update_specified.store(true, Ordering::Relaxed);
    }

    pub fn has_foreign_key(&self, fields: Vec<&Field>) -> bool {
        if self.through().is_some() {
            false
        } else {
            self.fields().iter().find(|name| fields.iter().find(|f| f.name() == name.as_str() && f.foreign_key()).is_some()).is_some()
        }
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

    pub(crate) fn build(self, fields: Vec<&Field>) -> Relation {
        let has_foreign_key = self.has_foreign_key(fields);
        let r#type = self.r#type();
        let delete_rule = if !self.inner.delete_specified.load(Ordering::Relaxed) {
            // set default delete rule
            if r#type.is_optional() {
                if !has_foreign_key {
                    Delete::NoAction
                } else {
                    Delete::Nullify
                }
            } else if r#type.is_array() {
                Delete::NoAction
            } else {
                Delete::Cascade
            }
        } else {
            self.delete()
        };
        let update_rule = if !self.inner.update_specified.load(Ordering::Relaxed) {
            // set default update rule
            if r#type.is_optional() {
                Update::Nullify
            } else if r#type.is_array() {
                Update::NoAction
            } else {
                Update::Update
            }
        } else {
            self.update()
        };
        let relation = Relation {
            inner: Arc::new(relation::Inner {
                name: self.inner.name.clone(),
                comment: self.inner.comment.clone(),
                r#type: self.inner.r#type.clone(),
                optionality: self.optionality(),
                model: self.model(),
                through: self.through(),
                local: self.local(),
                foreign: self.foreign(),
                is_vec: self.is_vec(),
                fields: self.fields(),
                references: self.references(),
                delete: delete_rule,
                update: update_rule,
                has_foreign_key,
                data: self.data(),
            })
        };
        relation
    }
}