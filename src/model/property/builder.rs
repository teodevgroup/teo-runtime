use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::database::r#type::DatabaseType;
use crate::model::field::Index;
use crate::optionality::Optionality;
use crate::pipeline::Pipeline;
use crate::Value;

pub struct Builder {
    inner: Arc<Inner>
}

pub struct Inner {
    pub name: String,
    pub comment: Option<Comment>,
    pub r#type: Type,
    pub column_name: Arc<Mutex<String>>,
    pub optionality: Arc<Mutex<Optionality>>,
    pub database_type: Arc<Mutex<DatabaseType>>,
    pub dependencies: Arc<Mutex<Vec<String>>>,
    pub setter: Arc<Mutex<Option<Pipeline>>>,
    pub getter: Arc<Mutex<Option<Pipeline>>>,
    pub input_omissible: AtomicBool,
    pub output_omissible: AtomicBool,
    pub cached: AtomicBool,
    pub index: Arc<Mutex<Option<Index>>>,
    pub data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {

    pub fn new(name: String, comment: Option<Comment>, r#type: Type) -> Self {
        Self {
            inner: Arc::new(Inner {
                name: name.clone(),
                comment,
                r#type,
                column_name: Arc::new(Mutex::new(name)),
                optionality: Arc::new(Mutex::new(Optionality::Optional)),
                database_type: Arc::new(Mutex::new(DatabaseType::Undetermined)),
                dependencies: Arc::new(Mutex::new(vec![])),
                setter: Arc::new(Mutex::new(None)),
                getter: Arc::new(Mutex::new(None)),
                input_omissible: AtomicBool::new(false),
                output_omissible: AtomicBool::new(false),
                cached: AtomicBool::new(false),
                index: Arc::new(Mutex::new(None)),
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

    pub fn column_name(&self) -> String {
        self.inner.column_name.lock().unwrap().clone()
    }

    pub fn set_column_name(&self, column_name: String) {
        *self.inner.column_name.lock().unwrap() = column_name;
    }

    pub fn optionality(&self) -> Optionality {
        self.inner.optionality.lock().unwrap().clone()
    }

    pub fn set_optionality(&self, optionality: Optionality) {
        *self.inner.optionality.lock().unwrap() = optionality;
    }

    pub fn database_type(&self) -> DatabaseType {
        self.inner.database_type.lock().unwrap().clone()
    }

    pub fn set_database_type(&self, database_type: DatabaseType) {
        *self.inner.database_type.lock().unwrap() = database_type;
    }

    pub fn dependencies(&self) -> Vec<String> {
        self.inner.dependencies.lock().unwrap().clone()
    }

    pub fn set_dependencies(&self, dependencies: Vec<String>) {
        *self.inner.dependencies.lock().unwrap() = dependencies;
    }

    pub fn setter(&self) -> Option<Pipeline> {
        self.inner.setter.lock().unwrap().clone()
    }

    pub fn set_setter(&self, setter: Option<Pipeline>) {
        *self.inner.setter.lock().unwrap() = setter;
    }

    pub fn getter(&self) -> Option<Pipeline> {
        self.inner.getter.lock().unwrap().clone()
    }

    pub fn set_getter(&self, getter: Option<Pipeline>) {
        *self.inner.getter.lock().unwrap() = getter;
    }

    pub fn input_omissible(&self) -> bool {
        self.inner.input_omissible.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_input_omissible(&self, input_omissible: bool) {
        self.inner.input_omissible.store(input_omissible, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn output_omissible(&self) -> bool {
        self.inner.output_omissible.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_output_omissible(&self, output_omissible: bool) {
        self.inner.output_omissible.store(output_omissible, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn cached(&self) -> bool {
        self.inner.cached.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_cached(&self, cached: bool) {
        self.inner.cached.store(cached, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn index(&self) -> Option<Index> {
        self.inner.index.lock().unwrap().clone()
    }

    pub fn set_index(&self, index: Option<Index>) {
        *self.inner.index.lock().unwrap() = index;
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