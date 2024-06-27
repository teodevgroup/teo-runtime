use std::collections::{BTreeMap, BTreeSet};
use std::ops::BitOr;
use std::sync::Arc;
use indexmap::IndexMap;
use maplit::{btreemap, btreeset};
use serde::Serialize;
use teo_parser::ast::model::ModelResolved;
use teo_parser::r#type::reference::Reference;
use teo_parser::r#type::synthesized_shape_reference::SynthesizedShapeReference;
use teo_parser::r#type::Type;
use teo_result::{Result, Error};
use crate::action::Action;
use crate::action::action::{AGGREGATE_HANDLER, COPY_HANDLER, COPY_MANY_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::comment::Comment;
use crate::model;
use crate::model::field::column_named::ColumnNamed;
use crate::model::field::Field;
use crate::model::field::indexable::Indexable;
use crate::traits::named::Named;
use crate::model::Index;
use crate::model::index::Item;
use crate::model::migration::Migration;
use crate::model::property::Property;
use crate::model::relation::delete::Delete;
use crate::model::relation::Relation;
use crate::namespace::Namespace;
use crate::pipeline::pipeline::Pipeline;
use crate::traits::documentable::Documentable;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Model {
    pub(super) inner: Arc<Inner>,
}

#[derive(Debug, Serialize)]
pub(super) struct Inner {
    pub(super) path: Vec<String>,
    pub(super) parser_path: Vec<usize>,
    pub(super) namespace_path: Vec<String>,
    pub(super) comment: Option<Comment>,
    #[serde(rename = "tableName")]
    pub(super) table_name: String,
    pub(super) actions: Vec<Action>,
    #[serde(rename = "generateClient")]
    pub(super) generate_client: bool,
    #[serde(rename = "generateEntity")]
    pub(super) generate_entity: bool,
    #[serde(rename = "showInStudio")]
    pub(super) show_in_studio: bool,
    #[serde(rename = "synthesizeShapes")]
    pub(super) synthesize_shapes: bool,
    pub(super) fields: IndexMap<String, Field>,
    pub(super) relations: IndexMap<String, Relation>,
    pub(super) properties: IndexMap<String, Property>,
    pub(super) indexes: IndexMap<String, Index>,
    #[serde(rename = "primaryIndex")]
    pub(super) primary_index: String,
    #[serde(rename = "beforeSave")]
    pub(super) before_save: Pipeline,
    #[serde(rename = "afterSave")]
    pub(super) after_save: Pipeline,
    #[serde(rename = "beforeDelete")]
    pub(super) before_delete: Pipeline,
    #[serde(rename = "afterDelete")]
    pub(super) after_delete: Pipeline,
    #[serde(rename = "canRead")]
    pub(super) can_read: Pipeline,
    #[serde(rename = "canMutate")]
    pub(super) can_mutate: Pipeline,
    pub(super) migration: Migration,
    pub(super) builtin_handlers: Vec<Action>,
    pub(super) data: BTreeMap<String, Value>,
    pub(super) cache: Cache,
}

impl PartialEq for Model {

    fn eq(&self, other: &Self) -> bool {
        self.inner.path == other.inner.path
    }
}

impl Model {

    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                path: vec![],
                parser_path: vec![],
                namespace_path: vec![],
                table_name: "".to_string(),
                generate_client: true,
                generate_entity: true,
                show_in_studio: true,
                synthesize_shapes: true,
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
                builtin_handlers: vec![],
            })
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn parser_path(&self) -> &Vec<usize> {
        &self.inner.parser_path
    }

    pub fn namespace_path(&self) -> &Vec<String> {
        &self.inner.namespace_path
        //self.path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }

    pub fn table_name(&self) -> &str {
        &self.inner.table_name
    }

    pub fn actions(&self) -> &Vec<Action> {
        &self.inner.actions
    }

    pub fn generate_client(&self) -> bool {
        self.inner.generate_client
    }

    pub fn generate_entity(&self) -> bool {
        self.inner.generate_entity
    }

    pub fn show_in_studio(&self) -> bool {
        self.inner.show_in_studio
    }

    pub fn synthesize_shapes(&self) -> bool {
        self.inner.synthesize_shapes
    }

    pub fn fields(&self) -> &IndexMap<String, Field> {
        &self.inner.fields
    }

    pub fn relations(&self) -> &IndexMap<String, Relation> {
        &self.inner.relations
    }

    pub fn properties(&self) -> &IndexMap<String, Property> {
        &self.inner.properties
    }

    pub fn indexes(&self) -> &IndexMap<String, Index> {
        &self.inner.indexes
    }

    pub fn primary_index_name(&self) -> &str {
        &self.inner.primary_index
    }

    pub fn primary_index(&self) -> Option<&Index> {
        self.indexes().values().find(|i| i.r#type().is_primary())
    }


    pub fn before_save(&self) -> &Pipeline {
        &self.inner.before_save
    }

    pub fn after_save(&self) -> &Pipeline {
        &self.inner.after_save
    }

    pub fn before_delete(&self) -> &Pipeline {
        &self.inner.before_delete
    }

    pub fn after_delete(&self) -> &Pipeline {
        &self.inner.after_delete
    }

    pub fn can_read(&self) -> &Pipeline {
        &self.inner.can_read
    }

    pub fn can_mutate(&self) -> &Pipeline {
        &self.inner.can_mutate
    }

    pub fn migration(&self) -> &Migration {
        &self.inner.migration
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.inner.data
    }

    pub fn cache(&self) -> &Cache {
        &self.inner.cache
    }

    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields().get(name).filter(|f| !f.dropped)
    }

    pub fn dropped_field(&self, name: &str) -> Option<&Field> {
        self.fields().get(name).filter(|f| f.dropped)
    }

    pub fn relation(&self, name: &str) -> Option<&Relation> {
        self.relations().get(name)
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.properties().get(name)
    }

    pub fn field_with_column_name(&self, name: &str) -> Option<&Field> {
        self.fields().iter().find(|f| f.column_name() == name).map(|f| *f)
    }

    pub fn property_with_column_name(&self, name: &str) -> Option<&Property> {
        self.properties().iter().find(|p| p.column_name() == name).map(|p| *p)
    }

    pub(crate) fn allowed_keys_for_aggregate(&self, name: &str) -> BTreeSet<&str> {
        match name {
            "_count" => self.cache().scalar_keys.iter().map(|k| k.as_str()).collect::<BTreeSet<&str>>().bitor(&btreeset!{"_all"}),
            "_min" | "_max" => self.cache().scalar_keys.iter().map(|k| k.as_str()).collect(),
            _ => self.cache().scalar_number_keys.iter().map(|k| k.as_str()).collect(),
        }
    }

    pub fn allows_drop_when_migrate(&self) -> bool {
        self.migration().drop
    }

    pub fn input_type_for_builtin_handler(&self, handler: Action) -> Type {
        match handler {
            FIND_UNIQUE_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::find_unique_args(self.as_type_reference())),
            FIND_FIRST_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::find_first_args(self.as_type_reference())),
            FIND_MANY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::find_many_args(self.as_type_reference())),
            CREATE_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::create_args(self.as_type_reference())),
            UPDATE_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::update_args(self.as_type_reference())),
            COPY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::copy_args(self.as_type_reference())),
            UPSERT_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::upsert_args(self.as_type_reference())),
            DELETE_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::delete_args(self.as_type_reference())),
            CREATE_MANY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::create_many_args(self.as_type_reference())),
            UPDATE_MANY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::update_many_args(self.as_type_reference())),
            COPY_MANY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::copy_many_args(self.as_type_reference())),
            DELETE_MANY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::delete_many_args(self.as_type_reference())),
            COUNT_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::count_args(self.as_type_reference())),
            AGGREGATE_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::aggregate_args(self.as_type_reference())),
            GROUP_BY_HANDLER => Type::SynthesizedShapeReference(SynthesizedShapeReference::group_by_args(self.as_type_reference())),
            _ => unreachable!()
        }
    }

    pub fn output_type_for_builtin_handler(&self, handler: Action, namespace: &Namespace) -> Type {
        let data = namespace.interface_at_path(&vec!["std", "Data"]).unwrap().as_type_reference();
        let data_meta = namespace.interface_at_path(&vec!["std", "DataMeta"]).unwrap().as_type_reference();
        let paging_info = namespace.interface_at_path(&vec!["std", "PagingInfo"]).unwrap().as_type_reference();
        match handler {
            FIND_UNIQUE_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            FIND_FIRST_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            FIND_MANY_HANDLER => {
                Type::InterfaceObject(data_meta, vec![
                    Type::Array(Box::new(Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference())))),
                    Type::InterfaceObject(paging_info, vec![])
                ])
            },
            CREATE_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            UPDATE_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            COPY_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            UPSERT_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            DELETE_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference()))
                ])
            },
            CREATE_MANY_HANDLER => {
                Type::InterfaceObject(data_meta, vec![
                    Type::Array(Box::new(Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference())))),
                    Type::InterfaceObject(paging_info, vec![])
                ])
            },
            UPDATE_MANY_HANDLER => {
                Type::InterfaceObject(data_meta, vec![
                    Type::Array(Box::new(Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference())))),
                    Type::InterfaceObject(paging_info, vec![])
                ])
            },
            COPY_MANY_HANDLER => {
                Type::InterfaceObject(data_meta, vec![
                    Type::Array(Box::new(Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference())))),
                    Type::InterfaceObject(paging_info, vec![])
                ])
            },
            DELETE_MANY_HANDLER => {
                Type::InterfaceObject(data_meta, vec![
                    Type::Array(Box::new(Type::SynthesizedShapeReference(SynthesizedShapeReference::result(self.as_type_reference())))),
                    Type::InterfaceObject(paging_info, vec![])
                ])
            },
            COUNT_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::Int64
                ])
            },
            AGGREGATE_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::aggregate_result(self.as_type_reference()))
                ])
            },
            GROUP_BY_HANDLER => {
                Type::InterfaceObject(data, vec![
                    Type::SynthesizedShapeReference(SynthesizedShapeReference::group_by_result(self.as_type_reference()))
                ])
            },
            _ => unreachable!()
        }
    }

    fn as_type_reference(&self) -> Reference {
        Reference::new(self.parser_path().clone(), self.path().clone())
    }
}

#[derive(Debug, Serialize)]
pub struct Cache {
    #[serde(rename = "allKeys")]
    pub all_keys: Vec<String>,
    #[serde(rename = "inputKeys")]
    pub input_keys: Vec<String>,
    #[serde(rename = "saveKeys")]
    pub save_keys: Vec<String>,
    #[serde(rename = "saveKeysAndVirtualKeys")]
    pub save_keys_and_virtual_keys: Vec<String>,
    #[serde(rename = "outputKeys")]
    pub output_keys: Vec<String>,
    #[serde(rename = "queryKeys")]
    pub query_keys: Vec<String>,
    #[serde(rename = "uniqueQueryKeys")]
    pub unique_query_keys: Vec<BTreeSet<String>>,
    #[serde(rename = "sortKeys")]
    pub sort_keys: Vec<String>,
    #[serde(rename = "autoKeys")]
    pub auto_keys: Vec<String>,
    #[serde(rename = "denyRelationKeys")]
    pub deny_relation_keys: Vec<String>,
    #[serde(rename = "scalarKeys")]
    pub scalar_keys: Vec<String>,
    #[serde(rename = "scalarNumberKeys")]
    pub scalar_number_keys: Vec<String>,
    #[serde(rename = "localOutputKeys")]
    pub local_output_keys: Vec<String>,
    #[serde(rename = "relationOutputKeys")]
    pub relation_output_keys: Vec<String>,
    #[serde(rename = "fieldPropertyMap")]
    pub field_property_map: BTreeMap<String, Vec<String>>,
    #[serde(rename = "hasVirtualFields")]
    pub has_virtual_fields: bool,
    pub shape: ModelResolved,
}

impl Cache {

    pub(crate) fn new() -> Self {
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
            shape: ModelResolved::new(),
        }
    }
}

impl Named for Model {

    fn name(&self) -> &str {
        self.path().last().map(|s| s.as_str()).unwrap()
    }
}

impl Documentable for Model {

    fn comment(&self) -> Option<&Comment> {
        self.comment()
    }

    fn kind(&self) -> &'static str {
        "model"
    }
}