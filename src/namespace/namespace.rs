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
use crate::app::data::AppData;
use crate::config::admin::Admin;
use crate::handler::Handler;
use crate::traits::named::Named;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub(super) inner: Arc<Inner>,
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
pub(super) struct Inner {
    pub(super) path: Vec<String>,
    pub(super) namespaces: BTreeMap<String, Namespace>,
    pub(super) structs: BTreeMap<String, Struct>,
    pub(super) models: BTreeMap<String, Model>,
    pub(super) enums: BTreeMap<String, Enum>,
    pub(super) interfaces: BTreeMap<String, Interface>,
    pub(super) model_decorators: BTreeMap<String, model::Decorator>,
    pub(super) model_field_decorators: BTreeMap<String, model::field::Decorator>,
    pub(super) model_relation_decorators: BTreeMap<String, model::relation::Decorator>,
    pub(super) model_property_decorators: BTreeMap<String, model::property::Decorator>,
    pub(super) enum_decorators: BTreeMap<String, r#enum::Decorator>,
    pub(super) enum_member_decorators: BTreeMap<String, r#enum::member::Decorator>,
    pub(super) interface_decorators: BTreeMap<String, interface::Decorator>,
    pub(super) interface_field_decorators: BTreeMap<String, interface::field::Decorator>,
    pub(super) handler_decorators: BTreeMap<String, handler::Decorator>,
    pub(super) pipeline_items: BTreeMap<String, pipeline::Item>,
    pub(super) handler_middlewares: BTreeMap<String, middleware::Definition>,
    pub(super) request_middlewares: BTreeMap<String, middleware::Definition>,
    pub(super) handlers: BTreeMap<String, Handler>,
    pub(super) handler_templates: BTreeMap<String, Handler>,
    pub(super) model_handler_groups: BTreeMap<String, handler::Group>,
    pub(super) handler_groups: BTreeMap<String, handler::Group>,
    pub(super) server: Option<Server>,
    pub(super) connector: Option<Connector>,
    pub(super) clients: BTreeMap<String, Client>,
    pub(super) entities: BTreeMap<String, Entity>,
    pub(super) debug: Option<Debug>,
    pub(super) admin: Option<Admin>,
    pub(super) handler_middlewares_block: Option<middleware::Block>,
    pub(super) request_middlewares_block: Option<middleware::Block>,
    pub(super) database: Option<Database>,
    pub(super) connector_reference: Option<Vec<String>>,
    #[serde(skip)]
    pub(super) connection: Arc<Mutex<Option<Arc<dyn Connection>>>>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(super) handler_middleware_stack: &'static dyn Middleware,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(super) request_middleware_stack: &'static dyn Middleware,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(super) handler_map: handler::Map,
    pub(super) model_opposite_relations_map: BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>,
    #[serde(skip)]
    pub(super) app_data: AppData,
}

impl Serialize for Namespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}

impl Namespace {

    pub fn is_main(&self) -> bool {
        self.inner.path.is_empty()
    }

    pub fn is_std(&self) -> bool {
        self.path().len() == 1 && self.path().first().unwrap().as_str() == "std"
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn namespaces(&self) -> &BTreeMap<String, Namespace> {
        &self.inner.namespaces
    }

    pub fn structs(&self) -> &BTreeMap<String, Struct> {
        &self.inner.structs
    }

    pub fn models(&self) -> &BTreeMap<String, Model> {
        &self.inner.models
    }

    pub fn enums(&self) -> &BTreeMap<String, Enum> {
        &self.inner.enums
    }

    pub fn interfaces(&self) -> &BTreeMap<String, Interface> {
        &self.inner.interfaces
    }

    pub fn model_decorators(&self) -> &BTreeMap<String, model::Decorator> {
        &self.inner.model_decorators
    }

    pub fn model_field_decorators(&self) -> &BTreeMap<String, model::field::Decorator> {
        &self.inner.model_field_decorators
    }

    pub fn model_relation_decorators(&self) -> &BTreeMap<String, model::relation::Decorator> {
        &self.inner.model_relation_decorators
    }

    pub fn model_property_decorators(&self) -> &BTreeMap<String, model::property::Decorator> {
        &self.inner.model_property_decorators
    }

    pub fn enum_decorators(&self) -> &BTreeMap<String, r#enum::Decorator> {
        &self.inner.enum_decorators
    }

    pub fn enum_member_decorators(&self) -> &BTreeMap<String, r#enum::member::Decorator> {
        &self.inner.enum_member_decorators
    }

    pub fn interface_decorators(&self) -> &BTreeMap<String, interface::Decorator> {
        &self.inner.interface_decorators
    }

    pub fn interface_field_decorators(&self) -> &BTreeMap<String, interface::field::Decorator> {
        &self.inner.interface_field_decorators
    }

    pub fn handler_decorators(&self) -> &BTreeMap<String, handler::Decorator> {
        &self.inner.handler_decorators
    }

    pub fn pipeline_items(&self) -> &BTreeMap<String, pipeline::Item> {
        &self.inner.pipeline_items
    }

    pub fn handler_middlewares(&self) -> &BTreeMap<String, middleware::Definition> {
        &self.inner.handler_middlewares
    }

    pub fn request_middlewares(&self) -> &BTreeMap<String, middleware::Definition> {
        &self.inner.request_middlewares
    }

    pub fn handlers(&self) -> &BTreeMap<String, Handler> {
        &self.inner.handlers
    }

    pub fn handler_groups(&self) -> &BTreeMap<String, handler::Group> {
        &self.inner.handler_groups
    }

    pub fn handler_templates(&self) -> &BTreeMap<String, Handler> {
        &self.inner.handler_templates
    }

    pub fn model_handler_groups(&self) -> &BTreeMap<String, handler::Group> {
        &self.inner.model_handler_groups
    }

    pub fn server(&self) -> Option<&Server> {
        self.inner.server.as_ref()
    }

    pub fn connector(&self) -> Option<&Connector> {
        self.inner.connector.as_ref()
    }

    pub fn connection(&self) -> Option<Arc<dyn Connection>> {
        self.inner.connection.lock().unwrap().as_ref().cloned()
    }

    pub fn set_connection(&self, connection: Option<Arc<dyn Connection>>) {
        *self.inner.connection.lock().unwrap() = connection;
    }

    pub fn admin(&self) -> Option<&Admin> {
        self.inner.admin.as_ref()
    }

    pub fn debug(&self) -> Option<&Debug> {
        self.inner.debug.as_ref()
    }

    pub fn handler_middlewares_block(&self) -> Option<&middleware::Block> {
        self.inner.handler_middlewares_block.as_ref()
    }

    pub fn request_middlewares_block(&self) -> Option<&middleware::Block> {
        self.inner.request_middlewares_block.as_ref()
    }

    pub fn entities(&self) -> &BTreeMap<String, Entity> {
        &self.inner.entities
    }

    pub fn clients(&self) -> &BTreeMap<String, Client> {
        &self.inner.clients
    }

    pub fn database(&self) -> Option<&Database> {
        self.inner.database.as_ref()
    }

    pub fn handler_middleware_stack(&self) -> &'static dyn Middleware {
        self.inner.handler_middleware_stack
    }

    pub fn request_middleware_stack(&self) -> &'static dyn Middleware {
        self.inner.request_middleware_stack
    }

    pub fn handler_map(&self) -> &handler::Map {
        &self.inner.handler_map
    }

    pub fn namespace(&self, name: &str) -> Option<&Namespace> {
        self.inner.namespaces.get(name)
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
            ns.inner.model_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_field_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.model_field_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_relation_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::relation::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.model_relation_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn model_property_decorator_at_path(&self, path: &Vec<String>) -> Option<&model::property::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.model_property_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_decorator_at_path(&self, path: &Vec<String>) -> Option<&r#enum::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.enum_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_member_decorator_at_path(&self, path: &Vec<String>) -> Option<&r#enum::member::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.enum_member_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_decorator_at_path(&self, path: &Vec<String>) -> Option<&interface::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.interface_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_field_decorator_at_path(&self, path: &Vec<String>) -> Option<&interface::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.interface_field_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn handler_decorator_at_path(&self, path: &Vec<String>) -> Option<&handler::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.handler_decorators.get(decorator_name)
        } else {
            None
        }
    }

    pub fn pipeline_item_at_path(&self, path: &Vec<String>) -> Option<&pipeline::Item> {
        let pipeline_item_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.pipeline_items.get(pipeline_item_name)
        } else {
            None
        }
    }

    pub fn struct_at_path(&self, path: &Vec<String>) -> Option<&Struct> {
        let struct_name = path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.structs.get(struct_name)
        } else {
            None
        }
    }

    pub fn enum_at_path(&self, path: &Vec<String>) -> Option<&Enum> {
        let enum_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.enums.get(enum_name)
        } else {
            None
        }
    }

    pub fn model_at_path(&self, path: &Vec<String>) -> Option<&Model> {
        let model_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.models.get(model_name)
        } else {
            None
        }
    }

    pub fn interface_at_path(&self, path: &Vec<String>) -> Option<&Interface> {
        let interface_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.interfaces.get(interface_name)
        } else {
            None
        }
    }

    pub fn handler_middleware_at_path(&self, path: &Vec<String>) -> Option<&middleware::Definition> {
        let middleware_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.handler_middlewares.get(middleware_name)
        } else {
            None
        }
    }

    pub fn request_middleware_at_path(&self, path: &Vec<String>) -> Option<&middleware::Definition> {
        let middleware_name = path.last().unwrap().deref();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.inner.request_middlewares.get(middleware_name)
        } else {
            None
        }
    }

    pub fn handler_template_at_path(&self, path: &Vec<String>) -> Option<&Handler> {
        let handler_name = path.last().unwrap().deref();
        if path.len() == 1 {
            self.inner.handler_templates.get(handler_name)
        } else {
            // try find a namespace first
            let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.inner.handler_templates.get(handler_name)
            } else {
                None
            }
        }
    }

    pub fn handler_at_path(&self, path: &Vec<String>) -> Option<&Handler> {
        let handler_name = path.last().unwrap().deref();
        if path.len() == 1 {
            self.inner.handlers.get(handler_name)
        } else {
            // try finding a namespace first
            let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.clone()).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.inner.handlers.get(handler_name)
            } else {
                // try finding in group
                let handler_name = path.last().unwrap().deref();
                let group_name = path.get(path.len() - 2).unwrap().deref();
                let namespace_path: Vec<String> = path.into_iter().rev().skip(2).rev().map(|i| i.clone()).collect();
                if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                    if let Some(group) = dest_namespace.inner.handler_groups.get(group_name) {
                        group.handler(handler_name)
                    } else if let Some(group) = dest_namespace.inner.model_handler_groups.get(group_name) {
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
        self.inner.connector_reference.as_ref()
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
        for model in self.inner.models.values() {
            result.push(model);
        }
        for n in self.inner.namespaces.values() {
            if !n.inner.connector.is_some() {
                result.extend(n.models_under_connector());
            }
        }
        result
    }

    /// Get relations of model defined by related model
    pub fn model_opposite_relations(&self, model: &Model) -> Vec<(&Model, &Relation)> {
        let result = self.inner.model_opposite_relations_map.get(model.path()).unwrap();
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
        result.extend(self.inner.models.values().filter(|m| f(*m)));
        for n in self.inner.namespaces.values() {
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
        result.extend(self.inner.enums.values().filter(|m| f(*m)));
        for n in self.inner.namespaces.values() {
            result.extend(n._collect_enums(f));
        }
        result
    }

    pub fn app_data(&self) -> &AppData {
        &self.inner.app_data
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