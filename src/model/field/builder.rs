use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use maplit::btreemap;
use teo_parser::ast::schema::Schema;
use teo_parser::availability::Availability;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::database::database::Database;
use crate::database::r#type::DatabaseType;
use crate::model::Field;
use crate::model::field::{field, Index, Migration};
use crate::model::field::indexable::{Indexable, SetIndex};
use crate::model::field::set_optional::SetOptional;
use crate::optionality::Optionality;
use crate::pipeline::Pipeline;
use crate::readwrite::read::Read;
use crate::readwrite::write::Write;
use crate::traits::named::Named;
use crate::Value;

pub struct Builder {
    inner: Arc<Inner>
}

struct Inner {
    name: String,
    comment: Option<Comment>,
    r#type: Type,
    availability: Availability,
    column_name: Arc<Mutex<String>>,
    foreign_key: AtomicBool,
    dropped: AtomicBool,
    migration: Arc<Mutex<Option<Migration>>>,
    database_type: Arc<Mutex<DatabaseType>>,
    optionality: Arc<Mutex<Optionality>>,
    copy: AtomicBool,
    read: Arc<Mutex<Read>>,
    write: Arc<Mutex<Write>>,
    atomic: AtomicBool,
    r#virtual: AtomicBool,
    input_omissible: AtomicBool,
    output_omissible: AtomicBool,
    index: Arc<Mutex<Option<Index>>>,
    queryable: AtomicBool,
    sortable: AtomicBool,
    auto: AtomicBool,
    auto_increment: AtomicBool,
    default: Arc<Mutex<Option<Value>>>,
    on_set: Arc<Mutex<Pipeline>>,
    on_save: Arc<Mutex<Pipeline>>,
    on_output: Arc<Mutex<Pipeline>>,
    can_mutate: Arc<Mutex<Pipeline>>,
    can_read: Arc<Mutex<Pipeline>>,
    data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {
    pub fn new(name: String, comment: Option<Comment>, r#type: Type, availability: Availability) -> Self {
        Self {
            inner: Arc::new(Inner {
                name: name.clone(),
                comment,
                r#type,
                availability,
                column_name: Arc::new(Mutex::new(name)),
                foreign_key: AtomicBool::new(false),
                dropped: AtomicBool::new(false),
                migration: Arc::new(Mutex::new(None)),
                database_type: Arc::new(Mutex::new(DatabaseType::Undetermined)),
                optionality: Arc::new(Mutex::new(Optionality::Required)),
                copy: AtomicBool::new(true),
                read: Arc::new(Mutex::new(Read::Read)),
                write: Arc::new(Mutex::new(Write::Write)),
                atomic: Default::default(),
                r#virtual: AtomicBool::new(false),
                input_omissible: AtomicBool::new(false),
                output_omissible: AtomicBool::new(false),
                index: Arc::new(Mutex::new(None)),
                queryable: AtomicBool::new(true),
                sortable: AtomicBool::new(true),
                auto: AtomicBool::new(false),
                auto_increment: AtomicBool::new(false),
                default: Arc::new(Mutex::new(None)),
                on_set: Arc::new(Mutex::new(Pipeline::new())),
                on_save: Arc::new(Mutex::new(Pipeline::new())),
                on_output: Arc::new(Mutex::new(Pipeline::new())),
                can_mutate: Arc::new(Mutex::new(Pipeline::new())),
                can_read: Arc::new(Mutex::new(Pipeline::new())),
                data: Arc::new(Mutex::new(btreemap! {})),
            })
        }
    }

    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    pub fn r#type(&self) -> &Type {
        &self.inner.r#type
    }

    pub fn availability(&self) -> Availability {
        self.inner.availability
    }

    pub fn column_name(&self) -> String {
        self.inner.column_name.lock().unwrap().clone()
    }

    pub fn set_column_name(&self, column_name: String) {
        *self.inner.column_name.lock().unwrap() = column_name;
    }

    pub fn foreign_key(&self) -> bool {
        self.inner.foreign_key.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_foreign_key(&self, foreign_key: bool) {
        self.inner.foreign_key.store(foreign_key, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn dropped(&self) -> bool {
        self.inner.dropped.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_dropped(&self, dropped: bool) {
        self.inner.dropped.store(dropped, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn migration(&self) -> Option<&Migration> {
        self.inner.migration.lock().unwrap().as_ref()
    }

    pub fn set_migration(&self, migration: Option<Migration>) {
        *self.inner.migration.lock().unwrap() = migration;
    }

    pub fn database_type(&self) -> DatabaseType {
        self.inner.database_type.lock().unwrap().clone()
    }

    pub fn set_database_type(&self, database_type: DatabaseType) {
        *self.inner.database_type.lock().unwrap() = database_type;
    }

    pub fn optionality(&self) -> Optionality {
        self.inner.optionality.lock().unwrap().clone()
    }

    pub fn set_optionality(&self, optionality: Optionality) {
        *self.inner.optionality.lock().unwrap() = optionality;
    }

    pub fn copy(&self) -> bool {
        self.inner.copy.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_copy(&self, copy: bool) {
        self.inner.copy.store(copy, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn read(&self) -> Read {
        self.inner.read.lock().unwrap().clone()
    }

    pub fn set_read(&self, read: Read) {
        *self.inner.read.lock().unwrap() = read;
    }

    pub fn write(&self) -> Write {
        self.inner.write.lock().unwrap().clone()
    }

    pub fn set_write(&self, write: Write) {
        *self.inner.write.lock().unwrap() = write;
    }

    pub fn atomic(&self) -> bool {
        self.inner.atomic.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_atomic(&self, atomic: bool) {
        self.inner.atomic.store(atomic, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn r#virtual(&self) -> bool {
        self.inner.r#virtual.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_virtual(&self, r#virtual: bool) {
        self.inner.r#virtual.store(r#virtual, std::sync::atomic::Ordering::Relaxed);
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

    pub fn index(&self) -> Option<Index> {
        self.inner.index.lock().unwrap().clone()
    }

    pub fn set_index(&self, index: Option<Index>) {
        *self.inner.index.lock().unwrap() = index;
    }

    pub fn queryable(&self) -> bool {
        self.inner.queryable.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_queryable(&self, queryable: bool) {
        self.inner.queryable.store(queryable, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn sortable(&self) -> bool {
        self.inner.sortable.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_sortable(&self, sortable: bool) {
        self.inner.sortable.store(sortable, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn auto(&self) -> bool {
        self.inner.auto.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_auto(&self, auto: bool) {
        self.inner.auto.store(auto, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn auto_increment(&self) -> bool {
        self.inner.auto_increment.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_auto_increment(&self, auto_increment: bool) {
        self.inner.auto_increment.store(auto_increment, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn default(&self) -> Option<Value> {
        self.inner.default.lock().unwrap().clone()
    }

    pub fn set_default(&self, default: Option<Value>) {
        *self.inner.default.lock().unwrap() = default;
    }

    pub fn on_set(&self) -> Pipeline {
        self.inner.on_set.lock().unwrap().clone()
    }

    pub fn set_on_set(&self, on_set: Pipeline) {
        *self.inner.on_set.lock().unwrap() = on_set;
    }

    pub fn on_save(&self) -> Pipeline {
        self.inner.on_save.lock().unwrap().clone()
    }

    pub fn set_on_save(&self, on_save: Pipeline) {
        *self.inner.on_save.lock().unwrap() = on_save;
    }

    pub fn on_output(&self) -> Pipeline {
        self.inner.on_output.lock().unwrap().clone()
    }

    pub fn set_on_output(&self, on_output: Pipeline) {
        *self.inner.on_output.lock().unwrap() = on_output;
    }

    pub fn can_mutate(&self) -> Pipeline {
        self.inner.can_mutate.lock().unwrap().clone()
    }

    pub fn set_can_mutate(&self, can_mutate: Pipeline) {
        *self.inner.can_mutate.lock().unwrap() = can_mutate;
    }

    pub fn can_read(&self) -> Pipeline {
        self.inner.can_read.lock().unwrap().clone()
    }

    pub fn set_can_read(&self, can_read: Pipeline) {
        *self.inner.can_read.lock().unwrap() = can_read;
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

    pub fn build(self, database: Database, schema: &Schema) -> Field {
        let mut field = Field {
            inner: Arc::new(field::Inner {
                name: self.inner.name.clone(),
                comment: self.inner.comment.clone(),
                r#type: self.inner.r#type.clone(),
                availability: self.inner.availability,
                column_name: self.inner.column_name.lock().unwrap().clone(),
                foreign_key: self.inner.foreign_key.load(std::sync::atomic::Ordering::Relaxed),
                dropped: self.inner.dropped.load(std::sync::atomic::Ordering::Relaxed),
                migration: self.inner.migration.lock().unwrap().clone(),
                database_type: {
                    let mut database_type = self.inner.database_type.lock().unwrap().clone();
                    // set default database type
                    if database_type.is_undetermined() {
                        database_type = database.default_database_type(self.r#type(), schema)?
                    }
                    database_type
                },
                optionality: self.inner.optionality.lock().unwrap().clone(),
                copy: {
                    // do not copy primary field and unique field
                    let mut copy = self.inner.copy.load(std::sync::atomic::Ordering::Relaxed);
                    if self.index().is_some() && self.index().unwrap().r#type().is_unique_or_primary() {
                        copy = false;
                    }
                    copy
                },
                read: self.inner.read.lock().unwrap().clone(),
                write: self.inner.write.lock().unwrap().clone(),
                atomic: self.inner.atomic.load(std::sync::atomic::Ordering::Relaxed),
                r#virtual: self.inner.r#virtual.load(std::sync::atomic::Ordering::Relaxed),
                input_omissible: self.inner.input_omissible.load(std::sync::atomic::Ordering::Relaxed),
                output_omissible: self.inner.output_omissible.load(std::sync::atomic::Ordering::Relaxed),
                index: self.inner.index.lock().unwrap().clone(),
                queryable: self.inner.queryable.load(std::sync::atomic::Ordering::Relaxed),
                sortable: self.inner.sortable.load(std::sync::atomic::Ordering::Relaxed),
                auto: self.inner.auto.load(std::sync::atomic::Ordering::Relaxed),
                auto_increment: self.inner.auto_increment.load(std::sync::atomic::Ordering::Relaxed),
                default: self.inner.default.lock().unwrap().clone(),
                on_set: self.inner.on_set.lock().unwrap().clone(),
                on_save: self.inner.on_save.lock().unwrap().clone(),
                on_output: self.inner.on_output.lock().unwrap().clone(),
                can_mutate: self.inner.can_mutate.lock().unwrap().clone(),
                can_read: self.inner.can_read.lock().unwrap().clone(),
                data: self.inner.data.lock().unwrap().clone(),
            })
        };
        field
    }
}

impl Named for Builder {
    fn name(&self) -> &str {
        self.inner.name.as_str()
    }
}

impl SetIndex for Builder {
    fn set_index(&self, index: Index) {
        self.inner.index.lock().unwrap().replace(index);
    }
}

impl SetOptional for Builder {
    fn set_optional(&self) {
        self.set_optionality(Optionality::Optional);
        self.set_input_omissible(true);
        self.set_output_omissible(true);
    }

    fn set_required(&self) {
        self.set_optionality(Optionality::Required);
    }
}