use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use indexmap::IndexMap;
use itertools::Itertools;
use teo_parser::ast::model::ModelResolved;
use teo_result::{Error, Result};
use crate::action::Action;
use crate::comment::Comment;
use crate::model::{Field, Index, Migration, Model, Property, Relation};
use crate::model::field::indexable::Indexable;
use crate::model::relation::delete::Delete;
use crate::pipeline::Pipeline;
use crate::{model, Value};
use crate::model::field::typed::Typed;
use crate::model::index::Item;
use crate::traits::named::Named;

pub struct Builder {
    inner: Arc<Inner>,
}

struct Inner {
    pub path: Vec<String>,
    pub namespace_path: Vec<String>,
    pub parser_path: Vec<usize>,
    pub comment: Option<Comment>,
    pub table_name: Arc<Mutex<String>>,
    pub actions: Arc<Mutex<Vec<Action>>>,
    pub generate_client: AtomicBool,
    pub generate_entity: AtomicBool,
    pub show_in_studio: AtomicBool,
    pub synthesize_shapes: AtomicBool,
    pub fields: Arc<Mutex<IndexMap<String, Field>>>,
    pub relations: Arc<Mutex<IndexMap<String, Relation>>>,
    pub properties: Arc<Mutex<IndexMap<String, Property>>>,
    pub indexes: Arc<Mutex<IndexMap<String, Index>>>,
    pub primary_index: Arc<Mutex<String>>,
    pub before_save: Arc<Mutex<Pipeline>>,
    pub after_save: Arc<Mutex<Pipeline>>,
    pub before_delete: Arc<Mutex<Pipeline>>,
    pub after_delete: Arc<Mutex<Pipeline>>,
    pub can_read: Arc<Mutex<Pipeline>>,
    pub can_mutate: Arc<Mutex<Pipeline>>,
    pub migration: Arc<Mutex<Migration>>,
    pub data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {
    pub fn new(path: Vec<String>, parser_path: Vec<usize>, comment: Option<Comment>) -> Self {
        let mut table_name_namespace_prefix = path.iter().take(path.len() - 1).join("_");
        if !table_name_namespace_prefix.is_empty() {
            table_name_namespace_prefix += "__";
        }
        let table_name = table_name_namespace_prefix + path.last().unwrap();
        let namespace_path = path.iter().take(path.len() - 1).map(|s| s.to_string()).collect();
        Self {
            inner: Arc::new(Inner {
                path,
                namespace_path,
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

    pub fn namespace_path(&self) -> &Vec<String> {
        &self.inner.namespace_path
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

    pub fn insert_field(&self, name: String, field: Field) {
        self.inner.fields.lock().unwrap().insert(name, field);
    }

    pub fn fields(&self) -> IndexMap<String, Field> {
        self.inner.fields.lock().unwrap().clone()
    }

    pub fn set_fields(&self, fields: IndexMap<String, Field>) {
        *self.inner.fields.lock().unwrap() = fields;
    }

    pub fn insert_relation(&self, name: String, relation: Relation) {
        self.inner.relations.lock().unwrap().insert(name, relation);
    }

    pub fn relations(&self) -> IndexMap<String, Relation> {
        self.inner.relations.lock().unwrap().clone()
    }

    pub fn set_relations(&self, relations: IndexMap<String, Relation>) {
        *self.inner.relations.lock().unwrap() = relations;
    }

    pub fn insert_property(&self, name: String, property: Property) {
        self.inner.properties.lock().unwrap().insert(name, property);
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

    pub(crate) fn build(self, shape: ModelResolved) -> Result<Model> {
        // set primary index if it is set through model decorator
        let mut primary_index_name = "".to_owned();
        for index in self.indexes().values() {
            if index.r#type().is_primary() {
                primary_index_name = index.name().to_string();
            }
        }
        if !primary_index_name.is_empty() {
            self.set_primary_index(primary_index_name);
        }

        // load index and set primary index
        let mut indexes_from_fields = vec![];
        for field in self.fields().values() {
            if field.index().is_some() {
                if let Some(index) = self.collect_field_index(field) {
                    indexes_from_fields.push(index);
                }
            }
        }
        for property in self.properties().values() {
            if property.index().is_some() {
                if let Some(index) = self.collect_field_index(property) {
                    indexes_from_fields.push(index);
                }
            }
        }
        for index in indexes_from_fields {
            if index.r#type().is_primary() {
                self.set_primary_index(index.name().to_owned());
            }
            self.insert_index(index.name().to_owned(), index);
        }
        if self.primary_index().is_empty() {
            Err(Error::new("model must have a primary index"))?;
        }

        // load caches

        let all_field_keys: Vec<String> = self.fields().values().map(|f| f.name().to_owned()).collect();
        let all_relation_keys: Vec<String> = self.relations().values().map(|r| r.name().to_owned()).collect();
        let all_property_keys: Vec<String> = self.properties().values().map(|p| p.name().to_owned()).collect();
        let mut all_keys = vec![];
        all_keys.extend(all_field_keys.clone());
        all_keys.extend(all_relation_keys.clone());
        all_keys.extend(all_property_keys.clone());
        let input_field_keys: Vec<String> = self.fields().values().filter(|&f| !f.write().is_no_write()).map(|f| f.name().to_string()).collect();
        let input_relation_keys = all_relation_keys.clone();
        let input_property_keys: Vec<String> = self.properties().values().filter(|p| p.setter().is_some()).map(|p| p.name().to_string()).collect();
        let mut input_keys = vec![];
        input_keys.extend(input_field_keys);
        input_keys.extend(input_relation_keys);
        input_keys.extend(input_property_keys);
        let field_save_keys: Vec<String> = self.fields().values().filter(|f| { !f.r#virtual() }).map(|f| f.name().to_string()).collect();
        let property_save_keys: Vec<String> = self.properties().values().filter(|p| p.cached()).map(|p| p.name().to_string()).collect();
        let mut save_keys = vec![];
        save_keys.extend(field_save_keys.clone());
        save_keys.extend(property_save_keys.clone());
        let mut save_keys_and_virtual_keys = vec![];
        save_keys_and_virtual_keys.extend(all_field_keys.clone());
        save_keys_and_virtual_keys.extend(property_save_keys);
        let output_field_keys: Vec<String> = self.fields().values().filter(|&f| { !f.read().is_no_read() }).map(|f| { f.name().to_string() }).collect();
        let output_relation_keys = all_relation_keys.clone();
        let output_property_keys: Vec<String> = self.properties().values().filter(|p| p.getter().is_some()).map(|p| p.name().to_string()).collect();
        let mut output_keys = vec![];
        output_keys.extend(output_field_keys.clone());
        output_keys.extend(output_relation_keys.clone());
        output_keys.extend(output_property_keys.clone());
        let mut output_field_keys_and_property_keys = vec![];
        output_field_keys_and_property_keys.extend(output_field_keys);
        output_field_keys_and_property_keys.extend(output_property_keys);
        let sort_keys: Vec<String> = self.fields().values().filter(|f| f.sortable()).map(|f| f.name().to_owned()).collect();
        let query_keys: Vec<String> = {
            let mut query_keys: Vec<String> = self.fields().values().filter(|f| f.queryable()).map(|f| f.name().to_owned()).collect();
            query_keys.extend(all_relation_keys.clone());
            query_keys
        };
        let unique_query_keys: Vec<BTreeSet<String>> = {
            let mut result = vec![];
            for index in self.indexes().values() {
                let set = BTreeSet::from_iter(index.items().iter().map(|i| {
                    i.field.clone()
                }));
                result.push(set);
            }
            result
        };
        let auto_keys: Vec<String> = self.fields()
            .values()
            .filter(|&f| { f.auto() || f.auto_increment() })
            .map(|f| f.name().to_string())
            .collect();
        let deny_relation_keys: Vec<String> = self.relations()
            .values()
            .filter(|&r| { r.delete() == Delete::Deny })
            .map(|r| r.name().to_string())
            .collect();
        let scalar_keys: Vec<String> = self.fields()
            .values()
            .map(|f| f.name().to_string())
            .collect();
        let scalar_number_keys: Vec<String> = self.fields()
            .values()
            .filter(|f| f.r#type().is_any_int_or_float() || f.r#type().is_decimal())
            .map(|f| f.name().to_string())
            .collect();
        // assign
        let mut cache = model::model::Cache::new();
        cache.all_keys = all_keys.clone();
        cache.input_keys = input_keys;
        cache.save_keys = save_keys;
        cache.save_keys_and_virtual_keys = save_keys_and_virtual_keys;
        cache.output_keys = output_keys;
        cache.query_keys = query_keys;
        cache.sort_keys = sort_keys;
        cache.unique_query_keys = unique_query_keys;
        cache.auto_keys = auto_keys;
        cache.deny_relation_keys = deny_relation_keys;
        cache.scalar_keys = scalar_keys;
        cache.scalar_number_keys = scalar_number_keys;
        cache.local_output_keys = output_field_keys_and_property_keys;
        cache.relation_output_keys = output_relation_keys;

        // field property map
        cache.field_property_map = {
            let mut map = BTreeMap::new();
            for property in self.properties().values() {
                if property.cached() {
                    for dependency in property.dependencies() {
                        if map.get(dependency).is_none() {
                            map.insert(dependency.clone(), vec![]);
                        }
                        map.get_mut(dependency).unwrap().push(property.name().to_string())
                    }
                }
            }
            map
        };
        cache.shape = shape;
        Ok(Model {
            inner: Arc::new(model::model::Inner {
                path: self.inner.path.clone(),
                parser_path: self.inner.parser_path.clone(),
                namespace_path: self.inner.path.iter().take(self.inner.path.len() - 1).map(|s| s.to_string()).collect(),
                comment: self.inner.comment.clone(),
                table_name: self.inner.table_name.lock().unwrap().clone(),
                actions: self.inner.actions.lock().unwrap().clone(),
                generate_client: self.inner.generate_client.load(std::sync::atomic::Ordering::Relaxed),
                generate_entity: self.inner.generate_entity.load(std::sync::atomic::Ordering::Relaxed),
                show_in_studio: self.inner.show_in_studio.load(std::sync::atomic::Ordering::Relaxed),
                synthesize_shapes: self.inner.synthesize_shapes.load(std::sync::atomic::Ordering::Relaxed),
                fields: self.inner.fields.lock().unwrap().clone(),
                relations: self.inner.relations.lock().unwrap().clone(),
                properties: self.inner.properties.lock().unwrap().clone(),
                indexes: self.inner.indexes.lock().unwrap().clone(),
                primary_index: self.inner.primary_index.lock().unwrap().clone(),
                before_save: self.inner.before_save.lock().unwrap().clone(),
                after_save: self.inner.after_save.lock().unwrap().clone(),
                before_delete: self.inner.before_delete.lock().unwrap().clone(),
                after_delete: self.inner.after_delete.lock().unwrap().clone(),
                can_read: self.inner.can_read.lock().unwrap().clone(),
                can_mutate: self.inner.can_mutate.lock().unwrap().clone(),
                migration: self.inner.migration.lock().unwrap().clone(),
                data: self.inner.data.lock().unwrap().clone(),
                cache,
                builtin_handlers: self.figure_out_builtin_handlers()
            }),
        })
    }

    fn figure_out_builtin_handlers(&self) -> Vec<Action> {
        let mut result = vec![];
        for action in Action::builtin_handlers() {
            result.push(*action);
        }
        result
    }

    pub fn collect_field_index<I>(&self, indexable: &I) -> Option<Index> where I: Indexable {
        if let Some(field_index) = indexable.index() {
            let name = indexable.name();
            let index = model::Index::new(field_index.r#type(), name.to_owned(), vec![
                Item::new(
                    field_index.name().to_owned(),
                    field_index.sort(),
                    field_index.length(),
                )
            ]);
            Some(index)
        } else {
            None
        }
    }
}