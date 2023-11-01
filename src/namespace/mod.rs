use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::Arc;
use maplit::btreemap;
use crate::{interface, middleware, model, model::Model, r#enum};
use crate::arguments::Arguments;
use crate::config::client::Client;
use crate::config::connector::Connector;
use crate::config::debug::Debug;
use crate::config::entity::Entity;
use crate::config::server::Server;
use crate::config::test::Test;
use crate::connection::connection::Connection;
use teo_result::Error;
use crate::handler;
use crate::interface::Interface;
use crate::model::field::Field;
use crate::model::property::Property;
use crate::model::relation::Relation;
use crate::r#enum::Enum;
use crate::r#enum::member::Member;
use crate::r#struct::Struct;
use crate::utils::next_path;
use teo_result::Result;
use crate::database::database::Database;
use crate::middleware::middleware::{empty_middleware, Middleware};
use crate::pipeline;
use crate::stdlib::load::load;
use educe::Educe;
use crate::handler::Handler;

#[derive(Educe)]
#[educe(Debug)]
pub struct Namespace {
    pub path: Vec<String>,
    pub namespaces: BTreeMap<String, Namespace>,
    pub structs: BTreeMap<String, Struct>,
    pub models: BTreeMap<String, Model>,
    pub enums: BTreeMap<String, Enum>,
    pub interfaces: BTreeMap<String, Interface>,
    pub model_decorators: BTreeMap<String, model::Decorator>,
    pub model_field_decorators: BTreeMap<String, model::field::Decorator>,
    pub model_relation_decorators: BTreeMap<String, model::relation::Decorator>,
    pub model_property_decorators: BTreeMap<String, model::property::Decorator>,
    pub enum_decorators: BTreeMap<String, r#enum::Decorator>,
    pub enum_member_decorators: BTreeMap<String, r#enum::member::Decorator>,
    pub interface_decorators: BTreeMap<String, interface::Decorator>,
    pub interface_field_decorators: BTreeMap<String, interface::field::Decorator>,
    pub handler_decorators: BTreeMap<String, handler::Decorator>,
    pub pipeline_items: BTreeMap<String, pipeline::Item>,
    pub middlewares: BTreeMap<String, middleware::Definition>,
    pub model_handler_groups: BTreeMap<String, handler::Group>,
    pub handler_groups: BTreeMap<String, handler::Group>,
    pub server: Option<Server>,
    pub connector: Option<Connector>,
    pub clients: BTreeMap<String, Client>,
    pub entities: BTreeMap<String, Entity>,
    pub debug: Option<Debug>,
    pub test: Option<Test>,
    pub middlewares_block: Option<middleware::Block>,
    pub database: Option<Database>,
    pub connector_reference: Option<Vec<String>>,
    pub connection: Option<Arc<dyn Connection>>,
    #[educe(Debug(ignore))]
    pub middleware_stack: &'static dyn Middleware,
    #[educe(Debug(ignore))]
    pub handler_map: handler::Map,
}

impl Namespace {

    /// Create a main namespace
    pub fn main() -> Self {
        Self::new(vec![])
    }

    fn new(path: Vec<String>) -> Self {
        Self {
            path,
            namespaces: btreemap!{},
            structs: btreemap!{},
            models: btreemap!{},
            enums: btreemap!{},
            interfaces: btreemap!{},
            model_decorators: btreemap!{},
            model_field_decorators: btreemap!{},
            model_relation_decorators: btreemap!{},
            model_property_decorators: btreemap!{},
            enum_decorators: btreemap!{},
            enum_member_decorators: btreemap!{},
            interface_decorators: btreemap! {},
            interface_field_decorators: btreemap! {},
            handler_decorators: btreemap! {},
            pipeline_items: btreemap!{},
            middlewares: btreemap! {},
            model_handler_groups: btreemap! {},
            handler_groups: btreemap! {},
            server: None,
            connector: None,
            clients: btreemap! {},
            entities: btreemap! {},
            debug: None,
            test: None,
            middlewares_block: None,
            database: None,
            connector_reference: None,
            connection: None,
            middleware_stack: empty_middleware(),
            handler_map: handler::Map::new(),
        }
    }

    pub fn load_standard_library(&mut self) -> Result<()> {
        if self.path.is_empty() {
            Err(Error::new("Standard library can only be loaded on main namespace"))?
        }
        load(self);
        Ok(())
    }

    pub fn is_main(&self) -> bool {
        self.path.is_empty()
    }

    pub fn is_std(&self) -> bool {
        self.path() == vec!["std"]
    }

    pub fn path(&self) -> Vec<&str> {
        self.path.iter().map(|s| s.as_str()).collect()
    }

    pub fn namespace_mut(&mut self, name: &str) -> Option<&mut Namespace> {
        self.namespaces.get_mut(name)
    }

    pub fn namespace_mut_or_create(&mut self, name: &str) -> &mut Namespace {
        if !self.namespaces.contains_key(name) {
            self.namespaces.insert(name.to_owned(), Namespace::new(next_path(&self.path, name)));
        }
        self.namespaces.get_mut(name).unwrap()
    }

    pub fn namespace_mut_or_create_at_path(&mut self, path: &Vec<&str>) -> &mut Namespace {
        let mut current = self;
        for item in path {
            current = current.namespace_mut_or_create(*item)
        }
        current
    }

    pub fn namespace(&self, name: &str) -> Option<&Namespace> {
        self.namespaces.get(name)
    }

    pub fn namespace_at_path(&self, path: &Vec<&str>) -> Option<&Namespace> {
        let mut current = Some(self);
        for item in path {
            if current.is_none() {
                return None;
            }
            current = current.unwrap().namespace(item);
        }
        current
    }

    pub fn define_model_decorator(&mut self, name: &str, call: fn(Arguments, &mut Model) -> Result<()>) {
        self.model_decorators.insert(name.to_owned(), model::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_model_field_decorator(&mut self, name: &str, call: fn(Arguments, &mut Field) -> Result<()>) {
        self.model_field_decorators.insert(name.to_owned(), model::field::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_model_relation_decorator(&mut self, name: &str, call: fn(Arguments, &mut Relation) -> Result<()>) {
        self.model_relation_decorators.insert(name.to_owned(), model::relation::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_model_property_decorator(&mut self, name: &str, call: fn(Arguments, &mut Property) -> Result<()>) {
        self.model_property_decorators.insert(name.to_owned(), model::property::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_enum_decorator(&mut self, name: &str, call: fn(Arguments, &mut Enum) -> Result<()>) {
        self.enum_decorators.insert(name.to_owned(), r#enum::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_enum_member_decorator(&mut self, name: &str, call: fn(Arguments, &mut Member) -> Result<()>) {
        self.enum_member_decorators.insert(name.to_owned(), r#enum::member::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_handler_decorator(&mut self, name: &str, call: fn(Arguments, &mut Handler) -> Result<()>) {
        self.handler_decorators.insert(name.to_owned(), handler::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_pipeline_item<T>(&mut self, name: &str, call: T) where T: pipeline::item::Call + 'static {
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(call)
        });
    }

    pub fn define_middleware<T>(&mut self, name: &str, call: T) where T: middleware::creator::Creator + 'static {
        self.middlewares.insert(name.to_owned(), middleware::Definition {
            path: next_path(&self.path, name),
            creator: Arc::new(call)
        });
    }

    pub fn define_model_handler_group<T>(&mut self, name: &str, builder: T) where T: Fn(&mut handler::Group) {
        let mut handler_group = handler::Group {
            path: next_path(&self.path, name),
            handlers: btreemap!{},
        };
        builder(&mut handler_group);
        self.model_handler_groups.insert(name.to_owned(), handler_group);
    }

    pub fn define_handler_group<T>(&mut self, name: &str, builder: T) where T: Fn(&mut handler::Group) {
        let mut handler_group = handler::Group {
            path: next_path(&self.path, name),
            handlers: btreemap!{},
        };
        builder(&mut handler_group);
        self.handler_groups.insert(name.to_owned(), handler_group);
    }

    pub fn define_struct<T>(&mut self, name: &str, builder: T) where T: Fn(&'static Vec<String>, &mut Struct) {
        let path = Box::leak(Box::new(next_path(&self.path, name))) as &'static Vec<String>;
        let mut r#struct = Struct {
            path: path.clone(),
            functions: btreemap! {},
            static_functions: btreemap! {}
        };
        builder(path, &mut r#struct);
        self.structs.insert(name.to_owned(), r#struct);
    }

    pub fn model_decorator_at_path(&self, path: &Vec<&str>) -> Option<&model::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_field_decorator_at_path(&self, path: &Vec<&str>) -> Option<&model::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_field_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_relation_decorator_at_path(&self, path: &Vec<&str>) -> Option<&model::relation::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_relation_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_property_decorator_at_path(&self, path: &Vec<&str>) -> Option<&model::property::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_property_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_decorator_at_path(&self, path: &Vec<&str>) -> Option<&r#enum::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enum_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_member_decorator_at_path(&self, path: &Vec<&str>) -> Option<&r#enum::member::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enum_member_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_decorator_at_path(&self, path: &Vec<&str>) -> Option<&interface::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_field_decorator_at_path(&self, path: &Vec<&str>) -> Option<&interface::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface_field_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn handler_decorator_at_path(&self, path: &Vec<&str>) -> Option<&handler::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.handler_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn pipeline_item_at_path(&self, path: &Vec<&str>) -> Option<&pipeline::Item> {
        let pipeline_item_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.pipeline_items.get(pipeline_item_name)
        } else {
            None
        }
    }

    pub fn struct_at_path(&self, path: &Vec<&str>) -> Option<&Struct> {
        let struct_name = *path.last().unwrap();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.structs.get(struct_name)
        } else {
            None
        }
    }

    pub fn enum_at_path(&self, path: &Vec<&str>) -> Option<&Enum> {
        let enum_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enums.get(enum_name)
        } else {
            None
        }
    }

    pub fn model_at_path(&self, path: &Vec<&str>) -> Option<&Model> {
        let model_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.models.get(model_name)
        } else {
            None
        }
    }

    pub fn interface_at_path(&self, path: &Vec<&str>) -> Option<&Interface> {
        let interface_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interfaces.get(interface_name)
        } else {
            None
        }
    }

    pub fn middleware_at_path(&self, path: &Vec<&str>) -> Option<&middleware::Definition> {
        let middleware_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.middlewares.get(middleware_name)
        } else {
            None
        }
    }

    pub fn handler_at_path(&self, path: &Vec<&str>) -> Option<&Handler> {
        let handler_name = path.last().unwrap().deref();
        let group_name = path.get(path.len() - 2).unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(2).rev().map(|i| *i).collect();
        if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
            if let Some(group) = dest_namespace.handler_groups.get(group_name) {
                group.handlers.get(handler_name)
            } else if let Some(group) = dest_namespace.model_handler_groups.get(group_name) {
                group.handlers.get(handler_name)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn replace_handler_at_path(&mut self, path: &Vec<&str>, handler: Handler) {
        let handler_name = path.last().unwrap().deref();
        let group_name = path.get(path.len() - 2).unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(2).rev().map(|i| *i).collect();
        let dest_namespace = self.namespace_mut_or_create_at_path(&namespace_path);
        if let Some(group) = dest_namespace.handler_groups.get_mut(group_name) {
            group.handlers.insert(handler_name.to_string(), handler);
        } else if let Some(group) = dest_namespace.model_handler_groups.get_mut(group_name) {
            group.handlers.insert(handler_name.to_string(), handler);
        }
    }

    pub fn connector_reference(&self) -> Option<Vec<&str>> {
        self.connector_reference.as_ref().map(|r| r.iter().map(AsRef::as_ref).collect())
    }

    /// Returns the opposite relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph.
    ///
    /// # Return Value
    ///
    /// A tuple of opposite relation's model and opposite relation.
    ///
    pub fn opposite_relation(&self, relation: &Relation) -> (&Model, Option<&Relation>) {
        let opposite_model = self.model_at_path(&relation.model_path()).unwrap();
        let opposite_relation = opposite_model.relations.values().find(|r| &r.fields == &relation.references && &r.references == &relation.fields);
        (opposite_model, opposite_relation)
    }

    /// Returns the through relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph. This relation must be a
    /// through relation.
    ///
    /// # Return Value
    ///
    /// A tuple of through relation's model and through model's local relation.
    ///
    pub fn through_relation(&self, relation: &Relation) -> (&Model, &Relation) {
        let through_model = self.model_at_path(&relation.through_path().unwrap()).unwrap();
        let through_local_relation = through_model.relation(relation.local.as_ref().unwrap()).unwrap();
        (through_model, through_local_relation)
    }

    /// Returns the through opposite relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph. This relation must be a
    /// through relation.
    ///
    /// # Return Value
    ///
    /// A tuple of through relation's model and through model's foreign relation.
    ///
    pub fn through_opposite_relation(&self, relation: &Relation) -> (&Model, &Relation) {
        let through_model = self.model_at_path(&relation.through_path().unwrap()).unwrap();
        let through_foreign_relation = through_model.relation(relation.foreign.as_ref().unwrap()).unwrap();
        (through_model, through_foreign_relation)
    }

    pub fn models_under_connector(&self) -> Vec<&Model> {
        let mut result = vec![];
                for model in self.models.values() {
            result.push(model);
        }
        for n in self.namespaces.values() {
            if !n.connector.is_some() {
                result.extend(n.models_under_connector());
            }
        }
        result
    }
}

unsafe impl Send for Namespace { }
unsafe impl Sync for Namespace { }