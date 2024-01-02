use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::Arc;
use maplit::btreemap;
use crate::{interface, middleware, model, model::Model, r#enum, request};
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
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::handler::ctx_argument::HandlerCtxArgument;
use crate::handler::Handler;
use crate::handler::handler::Method;
use crate::object::Object;
use crate::pipeline::item::callback::{CallbackArgument, CallbackResult};
use crate::pipeline::item::compare::CompareArgument;
use crate::pipeline::item::transform::{TransformArgument, TransformResult};
use crate::pipeline::item::validator::{ValidateArgument, ValidateResult, Validity};
use crate::traits::named::Named;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
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
    #[serde(skip)]
    pub connection: Option<Arc<dyn Connection>>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub middleware_stack: &'static dyn Middleware,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub handler_map: handler::Map,
    pub model_opposite_relations_map: BTreeMap<Vec<String>, Vec<(Vec<String>, String)>> // model path, relation name
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
            handlers: btreemap!{},
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
            model_opposite_relations_map: btreemap! {},
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

    pub fn namespace(&self, name: &str) -> Option<&Namespace> {
        self.namespaces.get(name)
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

    pub fn namespace_mut_at_path(&mut self, path: &Vec<&str>) -> Option<&mut Namespace> {
        let mut current = Some(self);
        for item in path {
            if current.is_none() {
                return None;
            }
            current = current.unwrap().namespace_mut(item);
        }
        current
    }

    pub fn namespace_mut_or_create_at_path(&mut self, path: &Vec<&str>) -> &mut Namespace {
        let mut current = self;
        for item in path {
            current = current.namespace_mut_or_create(*item)
        }
        current
    }

    pub fn define_model_decorator<F>(&mut self, name: &str, call: F) where F: Fn(Arguments, &mut Model) -> Result<()> + 'static {
        self.model_decorators.insert(name.to_owned(), model::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_model_field_decorator(&mut self, name: &str, call: impl Fn(Arguments, &mut Field) -> Result<()> + 'static) {
        self.model_field_decorators.insert(name.to_owned(), model::field::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_model_relation_decorator(&mut self, name: &str, call: impl Fn(Arguments, &mut Relation) -> Result<()> + 'static) {
        self.model_relation_decorators.insert(name.to_owned(), model::relation::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_model_property_decorator(&mut self, name: &str, call: impl Fn(Arguments, &mut Property) -> Result<()> + 'static) {
        self.model_property_decorators.insert(name.to_owned(), model::property::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_enum_decorator(&mut self, name: &str, call: impl Fn(Arguments, &mut Enum) -> Result<()> + 'static) {
        self.enum_decorators.insert(name.to_owned(), r#enum::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_enum_member_decorator(&mut self, name: &str, call: impl Fn(Arguments, &mut Member) -> Result<()> + 'static) {
        self.enum_member_decorators.insert(name.to_owned(), r#enum::member::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_handler_decorator(&mut self, name: &str, call: impl Fn(Arguments, &mut Handler) -> Result<()> + 'static) {
        self.handler_decorators.insert(name.to_owned(), handler::Decorator { path: next_path(&self.path, name), call: Arc::new(call) });
    }

    pub fn define_pipeline_item<T>(&mut self, name: &str, call: T) where T: pipeline::item::Call + 'static {
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(call)
        });
    }

    pub fn define_transform_pipeline_item<A, O, F, R>(&mut self, name: &str, call: F) where
        A: Send + Sync + 'static,
        O: Into<Object> + Send + Sync + 'static,
        R: Into<TransformResult<O>> + Send + Sync + 'static,
        F: TransformArgument<A, O, R> + 'static {
        let wrap_call = Box::leak(Box::new(call));
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                let transform_result: TransformResult<O> = wrap_call.call(ctx).await.into();
                match transform_result {
                    TransformResult::Object(t) => Ok(t.into()),
                    TransformResult::Result(r) => match r {
                        Ok(t) => Ok(t.into()),
                        Err(e) => Err(e.into()),
                    }
                }
            })
        });
    }

    pub fn define_validator_pipeline_item<T, F, O>(&mut self, name: &str, call: F) where
        T: Send + Sync + 'static,
        F: ValidateArgument<T, O> + 'static,
        O: Into<ValidateResult> + Send + Sync + 'static {
        let wrap_call = Box::leak(Box::new(call));
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                let ctx_value = ctx.value().clone();
                let validate_result: ValidateResult = wrap_call.call(ctx).await.into();
                match validate_result {
                    ValidateResult::Validity(validity) => if validity.is_valid() {
                        Ok(ctx_value)
                    } else if let Some(reason) = validity.invalid_reason() {
                        Err(Error::new(reason))
                    } else {
                        Err(Error::new("value is invalid"))
                    },
                    ValidateResult::Result(result) => match result {
                        Ok(validity) => if validity.is_valid() {
                            Ok(ctx_value)
                        } else if let Some(reason) = validity.invalid_reason() {
                            Err(Error::new(reason))
                        } else {
                            Err(Error::new("value is invalid"))
                        },
                        Err(err) => Err(err),
                    }
                }
            })
        });
    }

    pub fn define_callback_pipeline_item<T, F, O>(&mut self, name: &str, call: F) where
        T: Send + Sync + 'static,
        F: CallbackArgument<T, O> + 'static,
        O: Into<CallbackResult> + Send + Sync + 'static {
        let wrap_call = Box::leak(Box::new(call));
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                let ctx_value = ctx.value().clone();
                let callback_result: CallbackResult = wrap_call.call(ctx).await.into();
                match callback_result {
                    CallbackResult::Result(t) => match t {
                        Ok(_) => Ok(ctx_value),
                        Err(err) => Err(err),
                    },
                }
            })
        });
    }

    pub fn define_compare_pipeline_item<T, O, F>(&mut self, name: &str, call: F) where
        T: Send + Sync + 'static,
        O: Into<ValidateResult> + Send + Sync + 'static,
        F: CompareArgument<T, O> + 'static {
        let wrap_call = Box::leak(Box::new(call));
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                if ctx.object().is_new() {
                    return Ok(ctx.value().clone());
                }
                let key = ctx.path()[ctx.path().len() - 1].as_key().unwrap();
                let previous_value = ctx.object().get_previous_value(key)?;
                let current_value = ctx.value().clone().as_teon().unwrap().clone();
                if previous_value == current_value {
                    return Ok(ctx.value().clone());
                }
                let ctx_value = ctx.value().clone();
                let validate_result: ValidateResult = wrap_call.call(previous_value, current_value, ctx).await.into();
                match validate_result {
                    ValidateResult::Validity(validity) => if validity.is_valid() {
                        Ok(ctx_value)
                    } else if let Some(reason) = validity.invalid_reason() {
                        Err(Error::new(reason))
                    } else {
                        Err(Error::new("value is invalid"))
                    },
                    ValidateResult::Result(result) => match result {
                        Ok(validity) => if validity.is_valid() {
                            Ok(ctx_value)
                        } else if let Some(reason) = validity.invalid_reason() {
                            Err(Error::new(reason))
                        } else {
                            Err(Error::new("value is invalid"))
                        },
                        Err(err) => Err(err),
                    }
                }
            })
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

    pub fn define_handler<T, F>(&mut self, name: &str, call: F) where T: 'static, F: 'static + HandlerCtxArgument<T> {
        let wrapped_call = Box::leak(Box::new(call));
        let handler = Handler {
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            nonapi: false,
            format: HandlerInputFormat::Json,
            path: next_path(&self.path, name),
            ignore_prefix: false,
            method: Method::Post,
            interface: None,
            url: None,
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                wrapped_call.call(ctx).await
            })),
        };
        self.handlers.insert(name.to_owned(), handler);
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
        if path.len() == 1 {
            self.handlers.get(handler_name)
        } else {
            // try find a namespace first
            let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.handlers.get(handler_name)
            } else {
                // try find in group
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
        }
    }

    pub fn replace_handler_at_path(&mut self, path: &Vec<&str>, handler: Handler, inside_group: bool) {
        let handler_name = path.last().unwrap().deref();
        let group_name = if inside_group {
            Some(path.get(path.len() - 2).unwrap().deref())
        } else {
            None
        };
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(if inside_group { 2 } else { 1 }).rev().map(|i| *i).collect();
        let dest_namespace = self.namespace_mut_or_create_at_path(&namespace_path);
        if let Some(group_name) = group_name {
            if let Some(group) = dest_namespace.handler_groups.get_mut(group_name) {
                group.handlers.insert(handler_name.to_string(), handler);
            } else if let Some(group) = dest_namespace.model_handler_groups.get_mut(group_name) {
                group.handlers.insert(handler_name.to_string(), handler);
            }
        } else {
            dest_namespace.handlers.insert(handler_name.to_string(), handler);
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

    /// Get relations of model defined by related model
    pub fn model_opposite_relations(&self, model: &Model) -> Vec<(&Model, &Relation)> {
        let result = self.model_opposite_relations_map.get(&model.path).unwrap();
        result.iter().map(|result| {
            let model = self.model_at_path(&result.0.iter().map(AsRef::as_ref).collect()).unwrap();
            let relation = model.relation(result.1.as_str()).unwrap();
            (model, relation)
        }).collect()
    }
}

impl Named for Namespace {

    fn name(&self) -> &str {
        if self.path().is_empty() {
            "main"
        } else {
            *self.path().last().unwrap()
        }
    }
}

unsafe impl Send for Namespace { }
unsafe impl Sync for Namespace { }