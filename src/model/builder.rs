use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use indexmap::IndexMap;
use crate::action::Action;
use crate::comment::Comment;
use crate::model::{field, Field, Index, Migration, property, Property, relation, Relation};
use crate::pipeline::Pipeline;
use crate::Value;

pub struct Builder {
    inner: Arc<Inner>,
}

struct Inner {
    pub path: Vec<String>,
    pub parser_path: Vec<usize>,
    pub comment: Option<Comment>,
    #[serde(rename = "tableName")]
    pub table_name: Arc<Mutex<String>>,
    pub actions: Arc<Mutex<Vec<Action>>>,
    #[serde(rename = "generateClient")]
    pub generate_client: AtomicBool,
    #[serde(rename = "generateEntity")]
    pub generate_entity: AtomicBool,
    #[serde(rename = "showInStudio")]
    pub show_in_studio: AtomicBool,
    #[serde(rename = "synthesizeShapes")]
    pub synthesize_shapes: AtomicBool,
    pub fields: Arc<Mutex<IndexMap<String, Field>>>,
    pub relations: Arc<Mutex<IndexMap<String, Relation>>>,
    pub properties: Arc<Mutex<IndexMap<String, Property>>>,
    pub indexes: Arc<Mutex<IndexMap<String, Index>>>,
    #[serde(rename = "primaryIndex")]
    pub primary_index: Arc<Mutex<String>>,
    #[serde(rename = "beforeSave")]
    pub before_save: Arc<Mutex<Pipeline>>,
    #[serde(rename = "afterSave")]
    pub after_save: Arc<Mutex<Pipeline>>,
    #[serde(rename = "beforeDelete")]
    pub before_delete: Arc<Mutex<Pipeline>>,
    #[serde(rename = "afterDelete")]
    pub after_delete: Arc<Mutex<Pipeline>>,
    #[serde(rename = "canRead")]
    pub can_read: Arc<Mutex<Pipeline>>,
    #[serde(rename = "canMutate")]
    pub can_mutate: Arc<Mutex<Pipeline>>,
    pub migration: Arc<Mutex<Migration>>,
    pub data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {
    pub fn new(path: Vec<String>, parser_path: Vec<usize>, comment: Option<Comment>) -> Self {
        let table_name = path.last().unwrap().to_string();
        Self {
            inner: Arc::new(Inner {
                path,
                parser_path,
                comment,
                table_name: Arc::new(Mutex::new(table_name)),
                actions: Arc::new(Mutex::new(vec![])),
                generate_client: AtomicBool::new(true),
                generate_entity: AtomicBool::new(true),
                show_in_studio: AtomicBool::new(true),
                synthesize_shapes: AtomicBool::new(true),
                fields: Arc::new(Mutex::new(Default::default())),
                relations: Arc::new(Mutex::new(Default::default())),
                properties: Arc::new(Mutex::new(Default::default())),
                indexes: Arc::new(Mutex::new(Default::default())),
                primary_index: Arc::new(Mutex::new("".to_string())),
                before_save: Arc::new(Mutex::new(Pipeline::new())),
                after_save: Arc::new(Mutex::new(Pipeline::new())),
                before_delete: Arc::new(Mutex::new(Pipeline::new())),
                after_delete: Arc::new(Mutex::new(Pipeline::new())),
                can_read: Arc::new(Mutex::new(Pipeline::new())),
                can_mutate: Arc::new(Mutex::new(Pipeline::new())),
                migration: Arc::new(Mutex::new(Default::default())),
                data: Arc::new(Mutex::new(Default::default())),
            })
        }
    }

    pub fn table_name(&self) -> String {
        self.inner.table_name.lock().unwrap().clone()
    }

    pub fn set_table_name(&self, table_name: String) {
        *self.inner.table_name.lock().unwrap() = table_name;
    }

    pub fn actions(&self) -> Vec<Action> {
        self.inner.actions.lock().unwrap().clone()
    }

    pub fn set_actions(&self, actions: Vec<Action>) {
        *self.inner.actions.lock().unwrap() = actions;
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

    pub fn show_in_studio(&self) -> bool {
        self.inner.show_in_studio.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_show_in_studio(&self, show_in_studio: bool) {
        self.inner.show_in_studio.store(show_in_studio, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn synthesize_shapes(&self) -> bool {
        self.inner.synthesize_shapes.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_synthesize_shapes(&self, synthesize_shapes: bool) {
        self.inner.synthesize_shapes.store(synthesize_shapes, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn fields(&self) -> IndexMap<String, Field> {
        self.inner.fields.lock().unwrap().clone()
    }

    pub fn set_fields(&self, fields: IndexMap<String, Field>) {
        *self.inner.fields.lock().unwrap() = fields;
    }

    pub fn relations(&self) -> IndexMap<String, Relation> {
        self.inner.relations.lock().unwrap().clone()
    }

    pub fn set_relations(&self, relations: IndexMap<String, Relation>) {
        *self.inner.relations.lock().unwrap() = relations;
    }

    pub fn properties(&self) -> IndexMap<String, Property> {
        self.inner.properties.lock().unwrap().clone()
    }

    pub fn set_properties(&self, properties: IndexMap<String, Property>) {
        *self.inner.properties.lock().unwrap() = properties;
    }

    pub fn insert_index(&self, name: String, index: Index) {
        self.inner.indexes.lock().unwrap().insert(name, index);
    }

    pub fn indexes(&self) -> IndexMap<String, Index> {
        self.inner.indexes.lock().unwrap().clone()
    }

    pub fn set_indexes(&self, indexes: IndexMap<String, Index>) {
        *self.inner.indexes.lock().unwrap() = indexes;
    }

    pub fn primary_index(&self) -> String {
        self.inner.primary_index.lock().unwrap().clone()
    }

    pub fn set_primary_index(&self, primary_index: String) {
        *self.inner.primary_index.lock().unwrap() = primary_index;
    }

    pub fn before_save(&self) -> Pipeline {
        self.inner.before_save.lock().unwrap().clone()
    }

    pub fn set_before_save(&self, before_save: Pipeline) {
        *self.inner.before_save.lock().unwrap() = before_save;
    }

    pub fn after_save(&self) -> Pipeline {
        self.inner.after_save.lock().unwrap().clone()
    }

    pub fn set_after_save(&self, after_save: Pipeline) {
        *self.inner.after_save.lock().unwrap() = after_save;
    }

    pub fn before_delete(&self) -> Pipeline {
        self.inner.before_delete.lock().unwrap().clone()
    }

    pub fn set_before_delete(&self, before_delete: Pipeline) {
        *self.inner.before_delete.lock().unwrap() = before_delete;
    }

    pub fn after_delete(&self) -> Pipeline {
        self.inner.after_delete.lock().unwrap().clone()
    }

    pub fn set_after_delete(&self, after_delete: Pipeline) {
        *self.inner.after_delete.lock().unwrap() = after_delete;
    }

    pub fn can_read(&self) -> Pipeline {
        self.inner.can_read.lock().unwrap().clone()
    }

    pub fn set_can_read(&self, can_read: Pipeline) {
        *self.inner.can_read.lock().unwrap() = can_read;
    }

    pub fn can_mutate(&self) -> Pipeline {
        self.inner.can_mutate.lock().unwrap().clone()
    }

    pub fn set_can_mutate(&self, can_mutate: Pipeline) {
        *self.inner.can_mutate.lock().unwrap() = can_mutate;
    }

    pub fn migration(&self) -> Migration {
        self.inner.migration.lock().unwrap().clone()
    }

    pub fn set_migration(&self, migration: Migration) {
        *self.inner.migration.lock().unwrap() = migration;
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
}