use std::collections::{BTreeMap, BTreeSet};
use std::ops::BitOr;
use indexmap::IndexMap;
use maplit::{btreemap, btreeset};
use serde::Serialize;
use teo_parser::ast::model::ModelShapeResolved;
use teo_result::{Result, Error};
use crate::action::Action;
use crate::comment::Comment;
use crate::model;
use crate::model::field::column_named::ColumnNamed;
use crate::model::field::Field;
use crate::model::field::indexable::Indexable;
use crate::model::field::named::Named;
use crate::model::Index;
use crate::model::index::Item;
use crate::model::migration::Migration;
use crate::model::property::Property;
use crate::model::relation::delete::Delete;
use crate::model::relation::Relation;
use crate::object::Object;
use crate::pipeline::pipeline::Pipeline;
use crate::previous::Previous;

#[derive(Debug, Serialize)]
pub struct Model {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub table_name: String,
    pub actions: Vec<Action>,
    pub generate_client: bool,
    pub generate_entity: bool,
    pub show_in_studio: bool,
    pub fields: IndexMap<String, Field>,
    pub relations: IndexMap<String, Relation>,
    pub properties: IndexMap<String, Property>,
    pub indexes: IndexMap<String, Index>,
    pub primary_index: String,
    pub before_save: Pipeline,
    pub after_save: Pipeline,
    pub before_delete: Pipeline,
    pub after_delete: Pipeline,
    pub can_read: Pipeline,
    pub can_mutate: Pipeline,
    pub migration: Migration,
    pub data: BTreeMap<String, Object>,
    pub cache: Cache,
}

impl PartialEq for Model {

    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Model {

    pub fn new() -> Self {
        Self {
            path: vec![],
            table_name: "".to_string(),
            generate_client: true,
            generate_entity: true,
            show_in_studio: true,
            comment: None,
            fields: Default::default(),
            relations: Default::default(),
            properties: Default::default(),
            indexes: Default::default(),
            primary_index: "".to_string(),
            before_save: Pipeline::new(),
            after_save: Pipeline::new(),
            before_delete: Pipeline::new(),
            after_delete: Pipeline::new(),
            can_read: Pipeline::new(),
            can_mutate: Pipeline::new(),
            actions: vec![],
            migration: Default::default(),
            data: btreemap! {},
            cache: Cache::new(),
        }
    }

    pub fn namespace_path(&self) -> Vec<&str> {
        self.path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.get(name).filter(|f| !f.dropped)
    }

    pub fn dropped_field(&self, name: &str) -> Option<&Field> {
        self.fields.get(name).filter(|f| f.dropped)
    }

    pub fn relation(&self, name: &str) -> Option<&Relation> {
        self.relations.get(name)
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.properties.get(name)
    }

    pub fn field_with_column_name(&self, name: &str) -> Option<&Field> {
        self.fields().iter().find(|f| f.column_name() == name).map(|f| *f)
    }

    pub fn property_with_column_name(&self, name: &str) -> Option<&Property> {
        self.properties().iter().find(|p| p.column_name() == name).map(|p| *p)
    }

    pub fn indexes(&self) -> Vec<&Index> {
        self.indexes.values().collect()
    }

    pub fn fields(&self) -> Vec<&Field> {
        self.fields.values().collect()
    }

    pub fn relations(&self) -> Vec<&Relation> {
        self.relations.values().collect()
    }

    pub fn properties(&self) -> Vec<&Property> {
        self.properties.values().collect()
    }

    pub fn collect_field_index<I>(&self, indexable: &I) -> Option<Index> where I: Indexable {
        if let Some(field_index) = indexable.index() {
            let name = indexable.name();
            let index = model::Index::new(field_index.r#type, name.to_owned(), vec![
                Item::new(
                    field_index.name.clone(),
                    field_index.sort,
                    field_index.length,
                )
            ]);
            Some(index)
        } else {
            None
        }
    }

    pub(crate) fn allowed_keys_for_aggregate(&self, name: &str) -> BTreeSet<&str> {
        match name {
            "_count" => self.cache.scalar_keys.iter().map(|k| k.as_str()).collect::<BTreeSet<&str>>().bitor(&btreeset!{"_all"}),
            "_min" | "_max" => self.cache.scalar_keys.iter().map(|k| k.as_str()).collect(),
            _ => self.cache.scalar_number_keys.iter().map(|k| k.as_str()).collect(),
        }
    }

    pub fn allows_drop_when_migrate(&self) -> bool {
        self.migration.drop
    }

    pub fn primary_index(&self) -> Option<&Index> {
        self.indexes.values().find(|i| i.r#type().is_primary())
    }

    pub fn finalize(&mut self) -> Result<()> {

        // set default table name
        if self.table_name.is_empty() {
            let mut namespace_prefix = self.namespace_path().join("_");
            if !namespace_prefix.is_empty() {
                namespace_prefix += "__";
            }
            self.table_name = namespace_prefix + self.path.last().unwrap();
        }

        // load index and set primary index
        let mut indexes_from_fields = vec![];
        for field in self.fields.values() {
            if field.index().is_some() {
                if let Some(index) = self.collect_field_index(field) {
                    indexes_from_fields.push(index);
                }
            }
        }
        for property in self.properties.values() {
            if property.index().is_some() {
                if let Some(index) = self.collect_field_index(property) {
                    indexes_from_fields.push(index);
                }
            }
        }
        for index in indexes_from_fields {
            if index.r#type().is_primary() {
                self.primary_index = index.name().to_owned();
            }
            self.indexes.insert(index.name().to_owned(), index);
        }
        if self.primary_index.is_empty() {
            Err(Error::new("model must have a primary index"))?;
        }

        // install previous for primary field

        let primary_index = self.indexes.get(&self.primary_index).unwrap();
        for item in primary_index.items() {
            if let Some(field) = self.fields.get_mut(&item.field) {
                field.previous = Previous::Keep;
            }
        }

        // load caches

        let all_field_keys: Vec<String> = self.fields.values().map(|f| f.name().to_owned()).collect();
        let all_relation_keys: Vec<String> = self.relations.values().map(|r| r.name().to_owned()).collect();
        let all_property_keys: Vec<String> = self.properties.values().map(|p| p.name().to_owned()).collect();
        let mut all_keys = vec![];
        all_keys.extend(all_field_keys.clone());
        all_keys.extend(all_relation_keys.clone());
        all_keys.extend(all_property_keys.clone());
        let input_field_keys: Vec<String> = self.fields.values().filter(|&f| !f.write.is_no_write()).map(|f| f.name.clone()).collect();
        let input_relation_keys = all_relation_keys.clone();
        let input_property_keys: Vec<String> = self.properties.values().filter(|p| p.setter.is_some()).map(|p| p.name.clone()).collect();
        let mut input_keys = vec![];
        input_keys.extend(input_field_keys);
        input_keys.extend(input_relation_keys);
        input_keys.extend(input_property_keys);
        let field_save_keys: Vec<String> = self.fields.values().filter(|f| { !f.r#virtual }).map(|f| f.name.clone()).collect();
        let property_save_keys: Vec<String> = self.properties.values().filter(|p| p.cached).map(|p| p.name.clone()).collect();
        let mut save_keys = vec![];
        save_keys.extend(field_save_keys.clone());
        save_keys.extend(property_save_keys.clone());
        let mut save_keys_and_virtual_keys = vec![];
        save_keys_and_virtual_keys.extend(all_field_keys.clone());
        save_keys_and_virtual_keys.extend(property_save_keys);
        let output_field_keys: Vec<String> = self.fields.values().filter(|&f| { !f.read.is_no_read() }).map(|f| { f.name.clone() }).collect();
        let output_relation_keys = all_relation_keys.clone();
        let output_property_keys: Vec<String> = self.properties.values().filter(|p| p.getter.is_some()).map(|p| p.name.clone()).collect();
        let mut output_keys = vec![];
        output_keys.extend(output_field_keys.clone());
        output_keys.extend(output_relation_keys.clone());
        output_keys.extend(output_property_keys.clone());
        let mut output_field_keys_and_property_keys = vec![];
        output_field_keys_and_property_keys.extend(output_field_keys);
        output_field_keys_and_property_keys.extend(output_property_keys);
        let sort_keys: Vec<String> = self.fields.values().filter(|f| f.sortable).map(|f| f.name().to_owned()).collect();
        let query_keys: Vec<String> = {
            let mut query_keys: Vec<String> = self.fields.values().filter(|f| f.queryable).map(|f| f.name().to_owned()).collect();
            query_keys.extend(all_relation_keys.clone());
            query_keys
        };
        let unique_query_keys: Vec<BTreeSet<String>> = {
            let mut result = vec![];
            for index in self.indexes.values() {
                let set = BTreeSet::from_iter(index.items().iter().map(|i| {
                    i.field.clone()
                }));
                result.push(set);
            }
            result
        };
        let auto_keys: Vec<String> = self.fields
            .values()
            .filter(|&f| { f.auto || f.auto_increment })
            .map(|f| f.name.clone())
            .collect();
        let deny_relation_keys: Vec<String> = self.relations
            .values()
            .filter(|&r| { r.delete == Delete::Deny })
            .map(|r| r.name.clone())
            .collect();
        let scalar_keys: Vec<String> = self.fields
            .values()
            .map(|f| f.name.clone())
            .collect();
        let scalar_number_keys: Vec<String> = self.fields
            .values()
            .filter(|f| f.r#type.is_any_int_or_float() || f.r#type.is_decimal())
            .map(|f| f.name.clone())
            .collect();
        // assign
        self.cache.all_keys = all_keys.clone();
        self.cache.input_keys = input_keys;
        self.cache.save_keys = save_keys;
        self.cache.save_keys_and_virtual_keys = save_keys_and_virtual_keys;
        self.cache.output_keys = output_keys;
        self.cache.query_keys = query_keys;
        self.cache.sort_keys = sort_keys;
        self.cache.unique_query_keys = unique_query_keys;
        self.cache.auto_keys = auto_keys;
        self.cache.deny_relation_keys = deny_relation_keys;
        self.cache.scalar_keys = scalar_keys;
        self.cache.scalar_number_keys = scalar_number_keys;
        self.cache.local_output_keys = output_field_keys_and_property_keys;
        self.cache.relation_output_keys = output_relation_keys;

        // field property map
        self.cache.field_property_map = {
            let mut map = BTreeMap::new();
            for property in self.properties.values() {
                if property.cached {
                    for dependency in &property.dependencies {
                        if map.get(dependency).is_none() {
                            map.insert(dependency.clone(), vec![]);
                        }
                        map.get_mut(dependency).unwrap().push(property.name.clone())
                    }
                }
            }
            map
        };
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Cache {
    pub all_keys: Vec<String>,
    pub input_keys: Vec<String>,
    pub save_keys: Vec<String>,
    pub save_keys_and_virtual_keys: Vec<String>,
    pub output_keys: Vec<String>,
    pub query_keys: Vec<String>,
    pub unique_query_keys: Vec<BTreeSet<String>>,
    pub sort_keys: Vec<String>,
    pub auto_keys: Vec<String>,
    pub deny_relation_keys: Vec<String>,
    pub scalar_keys: Vec<String>,
    pub scalar_number_keys: Vec<String>,
    pub local_output_keys: Vec<String>,
    pub relation_output_keys: Vec<String>,
    pub field_property_map: BTreeMap<String, Vec<String>>,
    pub has_virtual_fields: bool,
    pub shape: ModelShapeResolved,
}

impl Cache {

    fn new() -> Self {
        Cache {
            all_keys: vec![],
            input_keys: vec![],
            save_keys: vec![],
            save_keys_and_virtual_keys: vec![],
            output_keys: vec![],
            query_keys: vec![],
            unique_query_keys: vec![],
            sort_keys: vec![],
            auto_keys: vec![],
            deny_relation_keys: vec![],
            scalar_keys: vec![],
            scalar_number_keys: vec![],
            local_output_keys: vec![],
            relation_output_keys: vec![],
            field_property_map: Default::default(),
            has_virtual_fields: false,
            shape: ModelShapeResolved::new(),
        }
    }
}

impl Named for Model {

    fn name(&self) -> &str {
        self.path.last().map(|s| s.as_str()).unwrap()
    }
}