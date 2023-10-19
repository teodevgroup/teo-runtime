use std::collections::{BTreeMap, BTreeSet};
use indexmap::IndexMap;
use serde::Serialize;
use crate::action::Action;
use crate::comment::Comment;
use crate::model::field::Field;
use crate::model::Index;
use crate::model::migration::Migration;
use crate::model::property::Property;
use crate::model::relation::Relation;
use crate::pipeline::pipeline::Pipeline;

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
    pub before_save: Vec<Pipeline>,
    pub after_save: Vec<Pipeline>,
    pub before_delete: Vec<Pipeline>,
    pub after_delete: Vec<Pipeline>,
    pub can_read: Vec<Pipeline>,
    pub can_mutate: Vec<Pipeline>,
    pub migration: Migration,
    pub cache: Cache,
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
            before_save: vec![],
            after_save: vec![],
            before_delete: vec![],
            after_delete: vec![],
            can_read: vec![],
            can_mutate: vec![],
            actions: vec![],
            migration: Default::default(),
            cache: Cache::new(),
        }
    }

    pub fn finalize(&mut self) {

    }
}

#[derive(Debug, Serialize)]
pub struct Cache {
    all_keys: Vec<String>,
    input_keys: Vec<String>,
    save_keys: Vec<String>,
    save_keys_and_virtual_keys: Vec<String>,
    output_keys: Vec<String>,
    query_keys: Vec<String>,
    unique_query_keys: Vec<BTreeSet<String>>,
    sort_keys: Vec<String>,
    auth_identity_keys: Vec<String>,
    auth_by_keys: Vec<String>,
    auto_keys: Vec<String>,
    deny_relation_keys: Vec<String>,
    scalar_keys: Vec<String>,
    scalar_number_keys: Vec<String>,
    local_output_keys: Vec<String>,
    relation_output_keys: Vec<String>,
    field_property_map: BTreeMap<String, Vec<String>>,
    has_virtual_fields: bool,
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
            auth_identity_keys: vec![],
            auth_by_keys: vec![],
            auto_keys: vec![],
            deny_relation_keys: vec![],
            scalar_keys: vec![],
            scalar_number_keys: vec![],
            local_output_keys: vec![],
            relation_output_keys: vec![],
            field_property_map: Default::default(),
            has_virtual_fields: false,
        }
    }
}