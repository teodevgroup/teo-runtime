use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use crate::{interface, middleware, model, model::Model, r#enum};
use crate::config::client::Client;
use crate::config::connector::Connector;
use crate::config::debug::Debug;
use crate::config::entity::Entity;
use crate::config::server::Server;
use crate::connection::connection::Connection;
use crate::handler;
use crate::interface::Interface;
use crate::model::relation::Relation;
use crate::r#enum::Enum;
use crate::r#struct::Struct;
use crate::database::database::Database;
use crate::middleware::middleware::Middleware;
use crate::pipeline;
use educe::Educe;
use serde::Serialize;
use crate::config::admin::Admin;
use crate::handler::Handler;
use crate::traits::named::Named;

#[derive(Educe, Serialize)]
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
    pub handlers: BTreeMap<String, Handler>,
    pub handler_templates: BTreeMap<String, Handler>,
    pub model_handler_groups: BTreeMap<String, handler::Group>,
    pub handler_groups: BTreeMap<String, handler::Group>,
    pub server: Option<Server>,
    pub connector: Option<Connector>,
    pub clients: BTreeMap<String, Client>,
    pub entities: BTreeMap<String, Entity>,
    pub debug: Option<Debug>,
    pub admin: Option<Admin>,
    pub middlewares_block: Option<middleware::Block>,
    pub database: Option<Database>,
    pub connector_reference: Option<Vec<String>>,
    #[serde(skip)]
    pub connection: Arc<Mutex<Option<Arc<dyn Connection>>>>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub middleware_stack: &'static dyn Middleware,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub handler_map: handler::Map,
    pub model_opposite_relations_map: BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>
}

impl Namespace {

    pub fn is_main(&self) -> bool {
        self.path.is_empty()
    }

    pub fn is_std(&self) -> bool {
        self.path().len() == 1 && self.path().first().unwrap().as_str() == "std"
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn namespace(&self, name: &str) -> Option<&Namespace> {
        self.namespaces.get(name)
    }

    pub fn namespace_at_path(&self, path: &Vec<String>) -> Option<&Namespace> {
        let mut current = Some(self);
        for item in path {
            if current.is_none() {
                return None;
            }
            current = current.unwrap().namespace(item);
        }
        current
    }

    pub fn model_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_field_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_field_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_relation_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::relation::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_relation_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_property_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::property::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_property_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_decorator_at_path(&self, path: &Vec<String>) -> Option<&r#enum::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enum_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_member_decorator_at_path(&self, path: &Vec<String>) -> Option<&r#enum::member::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enum_member_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_decorator_at_path(&self, path: &Vec<String>) -> Option<&interface::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_field_decorator_at_path(&self, path: &Vec<String>) -> Option<&interface::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface_field_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn handler_decorator_at_path(&self, path: &Vec<String>) -> Option<&handler::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.handler_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn pipeline_item_at_path(&self, path: &Vec<String>) -> Option<&pipeline::Item> {
        let pipeline_item_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.pipeline_items.get(pipeline_item_name)
        } else {
            None
        }
    }

    pub fn struct_at_path(&self, path: &Vec<String>) -> Option<&Struct> {
        let struct_name = path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.structs.get(struct_name)
        } else {
            None
        }
    }

    pub fn enum_at_path(&self, path: &Vec<String>) -> Option<&Enum> {
        let enum_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enums.get(enum_name)
        } else {
            None
        }
    }

    pub fn model_at_path(&self, path: &Vec<String>) -> Option<&Model> {
        let model_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.models.get(model_name)
        } else {
            None
        }
    }

    pub fn interface_at_path(&self, path: &Vec<String>) -> Option<&Interface> {
        let interface_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interfaces.get(interface_name)
        } else {
            None
        }
    }

    pub fn middleware_at_path(&self, path: &Vec<String>) -> Option<&middleware::Definition> {
        let middleware_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.middlewares.get(middleware_name)
        } else {
            None
        }
    }

    pub fn handler_template_at_path(&self, path: &Vec<String>) -> Option<&Handler> {
        let handler_name = path.last().unwrap().deref();
        if path.len() == 1 {
            self.handler_templates.get(handler_name)
        } else {
            // try find a namespace first
            let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.handler_templates.get(handler_name)
            } else {
                None
            }
        }
    }

    pub fn handler_at_path(&self, path: &Vec<String>) -> Option<&Handler> {
        let handler_name = path.last().unwrap().deref();
        if path.len() == 1 {
            self.handlers.get(handler_name)
        } else {
            // try finding a namespace first
            let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.clone()).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.handlers.get(handler_name)
            } else {
                // try finding in group
                let handler_name = path.last().unwrap().deref();
                let group_name = path.get(path.len() - 2).unwrap().deref();
                let namespace_path: Vec<String> = path.into_iter().rev().skip(2).rev().map(|i| i.clone()).collect();
                if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                    if let Some(group) = dest_namespace.handler_groups.get(group_name) {
                        group.handler(handler_name)
                    } else if let Some(group) = dest_namespace.model_handler_groups.get(group_name) {
                        group.handler(handler_name)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn connector_reference(&self) -> Option<&Vec<String>> {
        self.connector_reference.as_ref()
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
        let opposite_relation = opposite_model.relations().values().find(|r| r.fields() == relation.references() && r.references() == relation.fields());
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
        let through_local_relation = through_model.relation(relation.local().unwrap()).unwrap();
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
        let through_foreign_relation = through_model.relation(relation.foreign().unwrap()).unwrap();
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

    /// Get relations of model defined by related model
    pub fn model_opposite_relations(&self, model: &Model) -> Vec<(&Model, &Relation)> {
        let result = self.model_opposite_relations_map.get(model.path()).unwrap();
        result.iter().map(|result| {
            let model = self.model_at_path(&result.0).unwrap();
            let relation = model.relation(result.1.as_str()).unwrap();
            (model, relation)
        }).collect()
    }

    pub fn collect_models<F>(&self, f: F) -> Vec<&Model> where F: Fn(&Model) -> bool {
        let filter = &f;
        self._collect_models(filter)
    }

    pub fn _collect_models<F>(&self, f: &F) -> Vec<&Model> where F: Fn(&Model) -> bool {
        let mut result = vec![];
        result.extend(self.models.values().filter(|m| f(*m)));
        for n in self.namespaces.values() {
            result.extend(n._collect_models(f));
        }
        return result
    }

    pub fn collect_enums<F>(&self, f: F) -> Vec<&Enum> where F: Fn(&Enum) -> bool {
        let filter = &f;
        self._collect_enums(filter)
    }

    pub fn _collect_enums<F>(&self, f: &F) -> Vec<&Enum> where F: Fn(&Enum) -> bool {
        let mut result = vec![];
        result.extend(self.enums.values().filter(|m| f(*m)));
        for n in self.namespaces.values() {
            result.extend(n._collect_enums(f));
        }
        return result
    }
}

impl Named for Namespace {

    fn name(&self) -> &str {
        if self.path().is_empty() {
            "main"
        } else {
            self.path().last().unwrap()
        }
    }
}

unsafe impl Send for Namespace { }
unsafe impl Sync for Namespace { }