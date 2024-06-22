use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use teo_result::{Error, Result};
use crate::interface::Interface;
use crate::{handler, interface, middleware, model, pipeline, r#enum, request, Value};
use crate::arguments::Arguments;
use crate::config::admin::Admin;
use crate::config::client::Client;
use crate::config::connector::Connector;
use crate::config::debug::Debug;
use crate::config::entity::Entity;
use crate::config::server::Server;
use crate::connection::connection::Connection;
use crate::database::database::Database;
use crate::handler::ctx_argument::HandlerCtxArgument;
use crate::handler::Handler;
use crate::handler::handler::Method;
use crate::model::{Field, Model, Property, Relation};
use crate::namespace::Namespace;
use crate::pipeline::item::callback::{CallbackArgument, CallbackResult};
use crate::pipeline::item::compare::CompareArgument;
use crate::pipeline::item::transform::{TransformArgument, TransformResult};
use crate::pipeline::item::validator::{ValidateArgument, ValidateResult};
use crate::r#enum::Enum;
use crate::r#enum::member::Member;
use crate::r#struct::Struct;
use crate::stdlib::load::load;
use crate::utils::next_path;

#[derive(Clone)]
pub struct NamespaceBuilder {
    inner: Arc<NamespaceBuilderInner>
}

struct NamespaceBuilderInner {
    pub path: Vec<String>,
    pub namespaces: Arc<Mutex<BTreeMap<String, NamespaceBuilder>>>,
    pub structs: Arc<Mutex<BTreeMap<String, Struct>>>,
    pub models: Arc<Mutex<BTreeMap<String, Model>>>,
    pub enums: Arc<Mutex<BTreeMap<String, Enum>>>,
    pub interfaces: Arc<Mutex<BTreeMap<String, Interface>>>,
    pub model_decorators: Arc<Mutex<BTreeMap<String, model::Decorator>>>,
    pub model_field_decorators: Arc<Mutex<BTreeMap<String, model::field::Decorator>>>,
    pub model_relation_decorators: Arc<Mutex<BTreeMap<String, model::relation::Decorator>>>,
    pub model_property_decorators: Arc<Mutex<BTreeMap<String, model::property::Decorator>>>,
    pub enum_decorators: Arc<Mutex<BTreeMap<String, r#enum::Decorator>>>,
    pub enum_member_decorators: Arc<Mutex<BTreeMap<String, r#enum::member::Decorator>>>,
    pub interface_decorators: Arc<Mutex<BTreeMap<String, interface::Decorator>>>,
    pub interface_field_decorators: Arc<Mutex<BTreeMap<String, interface::field::Decorator>>>,
    pub handler_decorators: Arc<Mutex<BTreeMap<String, handler::Decorator>>>,
    pub pipeline_items: Arc<Mutex<BTreeMap<String, pipeline::Item>>>,
    pub middlewares: Arc<Mutex<BTreeMap<String, middleware::Definition>>>,
    pub handlers: Arc<Mutex<BTreeMap<String, Handler>>>,
    pub handler_templates: Arc<Mutex<BTreeMap<String, Handler>>>,
    pub model_handler_groups: Arc<Mutex<BTreeMap<String, handler::Group>>>,
    pub handler_groups: Arc<Mutex<BTreeMap<String, handler::Group>>>,
    pub server: Arc<Mutex<Option<Server>>>,
    pub connector: Arc<Mutex<Option<Connector>>>,
    pub clients: Arc<Mutex<BTreeMap<String, Client>>>,
    pub entities: Arc<Mutex<BTreeMap<String, Entity>>>,
    pub debug: Arc<Mutex<Option<Debug>>>,
    pub admin: Arc<Mutex<Option<Admin>>>,
    pub middlewares_block: Arc<Mutex<Option<middleware::Block>>>,
    pub database: Arc<Mutex<Option<Database>>>,
    pub connector_reference: Arc<Mutex<Option<Vec<String>>>>,
    pub connection: Arc<Mutex<Option<Arc<dyn Connection>>>>,
    pub model_opposite_relations_map: Arc<Mutex<BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>>>
}

impl NamespaceBuilder {
    pub fn main() -> Self {
        Self {
            inner: Arc::new(NamespaceBuilderInner {
                path: vec![],
                namespaces: Arc::new(Mutex::new(Default::default())),
                structs: Arc::new(Mutex::new(Default::default())),
                models: Arc::new(Mutex::new(Default::default())),
                enums: Arc::new(Mutex::new(Default::default())),
                interfaces: Arc::new(Mutex::new(Default::default())),
                model_decorators: Arc::new(Mutex::new(Default::default())),
                model_field_decorators: Arc::new(Mutex::new(Default::default())),
                model_relation_decorators: Arc::new(Mutex::new(Default::default())),
                model_property_decorators: Arc::new(Mutex::new(Default::default())),
                enum_decorators: Arc::new(Mutex::new(Default::default())),
                enum_member_decorators: Arc::new(Mutex::new(Default::default())),
                interface_decorators: Arc::new(Mutex::new(Default::default())),
                interface_field_decorators: Arc::new(Mutex::new(Default::default())),
                handler_decorators: Arc::new(Mutex::new(Default::default())),
                pipeline_items: Arc::new(Mutex::new(Default::default())),
                middlewares: Arc::new(Mutex::new(Default::default())),
                handlers: Arc::new(Mutex::new(Default::default())),
                handler_templates: Arc::new(Mutex::new(Default::default())),
                model_handler_groups: Arc::new(Mutex::new(Default::default())),
                handler_groups: Arc::new(Mutex::new(Default::default())),
                server: Arc::new(Mutex::new(None)),
                connector: Arc::new(Mutex::new(None)),
                clients: Arc::new(Mutex::new(Default::default())),
                entities: Arc::new(Mutex::new(Default::default())),
                debug: Arc::new(Mutex::new(None)),
                admin: Arc::new(Mutex::new(None)),
                middlewares_block: Arc::new(Mutex::new(None)),
                database: Arc::new(Mutex::new(None)),
                connector_reference: Arc::new(Mutex::new(None)),
                connection: Arc::new(Mutex::new(None)),
                model_opposite_relations_map: Arc::new(Mutex::new(Default::default())),
            })
        }
    }

    pub fn load_standard_library(&self) -> teo_result::Result<()> {
        if self.inner.path.is_empty() {
            Err(Error::new("Standard library can only be loaded on main namespace"))?
        }
        load(self);
        Ok(())
    }

    pub fn path(&self) -> Vec<&str> {
        self.inner.path.iter().map(|s| s.as_str()).collect()
    }

    pub fn is_main(&self) -> bool {
        self.path().is_empty()
    }

    pub fn is_std(&self) -> bool {
        self.path() == vec!["std"]
    }

    pub fn namespace(&self, name: &str) -> Option<NamespaceBuilder> {
        let namespaces = self.inner.namespaces.lock().unwrap();
        if let Some(namespace) = namespaces.get(name) {
            Some(namespace.clone())
        } else {
            None
        }
    }

    pub fn namespace_or_create(&self, name: &str) -> NamespaceBuilder {
        let mut namespaces = self.inner.namespaces.lock().unwrap();
        if !namespaces.contains_key(name) {
            namespaces.insert(name.to_owned(), Namespace::new(next_path(&self.inner.path, name)));
        }
        namespaces.get(name).unwrap().cloned()
    }

    pub fn namespace_at_path(&self, path: &Vec<&str>) -> Option<NamespaceBuilder> {
        let mut current = Some(self.clone());
        for item in path {
            if current.is_none() {
                return None;
            }
            current = current.unwrap().namespace(item);
        }
        current
    }

    pub fn namespace_or_create_at_path(&self, path: &Vec<&str>) -> NamespaceBuilder {
        let mut current = self.clone();
        for item in path {
            current = current.namespace_or_create(*item)
        }
        current
    }

    pub fn define_model_decorator<F>(&self, name: &str, call: F) where F: Fn(Arguments, &mut Model) -> Result<()> + 'static {
        let mut model_decorators = self.inner.model_decorators.lock().unwrap();
        model_decorators.insert(name.to_owned(), model::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_model_field_decorator(&self, name: &str, call: impl Fn(Arguments, &mut Field) -> Result<()> + 'static) {
        let mut model_field_decorators = self.inner.model_field_decorators.lock().unwrap();
        model_field_decorators.insert(name.to_owned(), model::field::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_model_relation_decorator(&self, name: &str, call: impl Fn(Arguments, &mut Relation) -> Result<()> + 'static) {
        let mut model_relation_decorators = self.inner.model_relation_decorators.lock().unwrap();
        model_relation_decorators.insert(name.to_owned(), model::relation::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_model_property_decorator(&self, name: &str, call: impl Fn(Arguments, &mut Property) -> Result<()> + 'static) {
        let mut model_property_decorators = self.inner.model_property_decorators.lock().unwrap();
        model_property_decorators.insert(name.to_owned(), model::property::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_enum_decorator(&self, name: &str, call: impl Fn(Arguments, &mut Enum) -> Result<()> + 'static) {
        let mut enum_decorators = self.inner.enum_decorators.lock().unwrap();
        enum_decorators.insert(name.to_owned(), r#enum::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_enum_member_decorator(&self, name: &str, call: impl Fn(Arguments, &mut Member) -> Result<()> + 'static) {
        let mut enum_member_decorators = self.inner.enum_member_decorators.lock().unwrap();
        enum_member_decorators.insert(name.to_owned(), r#enum::member::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_interface_decorator<F>(&self, name: &str, call: F) where F: Fn(Arguments, &mut Interface) -> Result<()> + 'static {
        let mut interface_decorators = self.inner.interface_decorators.lock().unwrap();
        interface_decorators.insert(name.to_owned(), interface::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_handler_decorator(&self, name: &str, call: impl Fn(Arguments, &mut Handler) -> Result<()> + 'static) {
        let mut handler_decorators = self.inner.handler_decorators.lock().unwrap();
        handler_decorators.insert(name.to_owned(), handler::Decorator { path: next_path(&self.inner.path, name), call: Arc::new(call) });
    }

    pub fn define_pipeline_item<T>(&self, name: &str, call: T) where T: pipeline::item::Call + 'static {
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.inner.path, name),
            call: Arc::new(call)
        });
    }

    pub fn define_transform_pipeline_item<A, O, F, R>(&self, name: &str, call: F) where
        A: Send + Sync + 'static,
        O: Into<Value> + Send + Sync + 'static,
        R: Into<TransformResult<O>> + Send + Sync + 'static,
        F: TransformArgument<A, O, R> + 'static {
        let wrap_call = Box::leak(Box::new(call));
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.inner.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                let transform_result: TransformResult<O> = wrap_call.call(args, ctx).await.into();
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

    pub fn define_validator_pipeline_item<T, F, O>(&self, name: &str, call: F) where
        T: Send + Sync + 'static,
        F: ValidateArgument<T, O> + 'static,
        O: Into<ValidateResult> + Send + Sync + 'static {
        let wrap_call = Box::leak(Box::new(call));
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.inner.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                let ctx_value = ctx.value().clone();
                let validate_result: ValidateResult = wrap_call.call(args, ctx).await.into();
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

    pub fn define_callback_pipeline_item<T, F, O>(&self, name: &str, call: F) where
        T: Send + Sync + 'static,
        F: CallbackArgument<T, O> + 'static,
        O: Into<CallbackResult> + Send + Sync + 'static {
        let wrap_call = Box::leak(Box::new(call));
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.inner.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                let ctx_value = ctx.value().clone();
                let callback_result: CallbackResult = wrap_call.call(args, ctx).await.into();
                match callback_result {
                    CallbackResult::Result(t) => match t {
                        Ok(_) => Ok(ctx_value),
                        Err(err) => Err(err),
                    },
                }
            })
        });
    }

    pub fn define_compare_pipeline_item<T, O, F, E>(&self, name: &str, call: F) where
        T: Send + Sync + 'static,
        O: Into<ValidateResult> + Send + Sync + 'static,
        E: Into<Error> + std::error::Error,
        F: CompareArgument<T, O, E> + 'static {
        let wrap_call = Box::leak(Box::new(call));
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.inner.path, name),
            call: Arc::new(|args: Arguments, ctx: pipeline::Ctx| async {
                if ctx.object().is_new() {
                    return Ok(ctx.value().clone());
                }
                let key = ctx.path()[ctx.path().len() - 1].as_key().unwrap();
                let previous_value = ctx.object().get_previous_value(key)?;
                let current_value = ctx.value().clone().clone();
                if previous_value == current_value {
                    return Ok(ctx.value().clone());
                }
                let ctx_value = ctx.value().clone();
                let validate_result: ValidateResult = wrap_call.call(previous_value, current_value, args, ctx).await.into();
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

    pub fn define_middleware<T>(&self, name: &str, call: T) where T: middleware::creator::Creator + 'static {
        let mut middlewares = self.inner.middlewares.lock().unwrap();
        middlewares.insert(name.to_owned(), middleware::Definition {
            path: next_path(&self.inner.path, name),
            creator: Arc::new(call)
        });
    }

    pub fn define_model_handler_group<T>(&self, name: &str, builder: T) where T: Fn(&handler::Group) {
        let handler_group = handler::Group::new(next_path(&self.inner.path, name));
        builder(&handler_group);
        let mut model_handler_groups = self.inner.model_handler_groups.lock().unwrap();
        model_handler_groups.insert(name.to_owned(), handler_group);
    }

    pub fn insert_handler(&self, name: &str, handler: Handler) {
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }

    pub fn define_handler<T, F>(&self, name: &str, call: F) where T: 'static, F: 'static + HandlerCtxArgument<T> {
        let wrapped_call = Box::leak(Box::new(call));
        let handler = Handler {
            namespace_path: self.inner.path.clone(),
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            nonapi: false,
            format: HandlerInputFormat::Json,
            path: next_path(&self.inner.path, name),
            ignore_prefix: false,
            method: Method::Post,
            interface: None,
            url: None,
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                wrapped_call.call(ctx).await
            })),
        };
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }

    pub fn define_handler_template<T, F>(&self, name: &str, call: F) where T: 'static, F: 'static + HandlerCtxArgument<T> {
        let wrapped_call = Box::leak(Box::new(call));
        let handler = Handler {
            namespace_path: self.inner.path.clone(),
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            nonapi: false,
            format: HandlerInputFormat::Json,
            path: next_path(&self.inner.path, name),
            ignore_prefix: false,
            method: Method::Post,
            interface: None,
            url: None,
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                wrapped_call.call(ctx).await
            })),
        };
        let mut handler_templates = self.inner.handler_templates.lock().unwrap();
        handler_templates.insert(name.to_owned(), handler);
    }

    pub fn define_handler_group<T>(&self, name: &str, builder: T) where T: Fn(&handler::Group) {
        let handler_group = handler::Group::new(next_path(&self.inner.path, name));
        builder(&handler_group);
        let mut handler_groups = self.inner.handler_groups.lock().unwrap();
        handler_groups.insert(name.to_owned(), handler_group);
    }

    pub fn define_struct<T>(&self, name: &str, builder: T) where T: Fn(&'static Vec<String>, &mut Struct) {
        let path = Box::leak(Box::new(next_path(&self.inner.path, name))) as &'static Vec<String>;
        let mut r#struct = Struct {
            path: path.clone(),
            functions: btreemap! {},
            static_functions: btreemap! {}
        };
        builder(path, &mut r#struct);
        let mut structs = self.inner.structs.lock().unwrap();
        structs.insert(name.to_owned(), r#struct);
    }

    pub fn model_decorator(&self, name: &str) -> Option<model::Decorator> {
        let model_decorators = self.inner.model_decorators.lock().unwrap();
        model_decorators.get(name).cloned()
    }

    pub fn model_decorator_at_path(&self, path: &Vec<&str>) -> Option<model::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn model_field_decorator(&self, name: &str) -> Option<model::field::Decorator> {
        let model_field_decorators = self.inner.model_field_decorators.lock().unwrap();
        model_field_decorators.get(name).cloned()
    }

    pub fn model_field_decorator_at_path(&self, path: &Vec<&str>) -> Option<model::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_field_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn model_relation_decorator(&self, name: &str) -> Option<model::relation::Decorator> {
        let model_relation_decorators = self.inner.model_relation_decorators.lock().unwrap();
        model_relation_decorators.get(name).cloned()
    }

    pub fn model_relation_decorator_at_path(&self, path: &Vec<&str>) -> Option<model::relation::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_relation_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn model_property_decorator(&self, name: &str) -> Option<model::property::Decorator> {
        let model_property_decorators = self.inner.model_property_decorators.lock().unwrap();
        model_property_decorators.get(name).cloned()
    }

    pub fn model_property_decorator_at_path(&self, path: &Vec<&str>) -> Option<model::property::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model_property_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_decorator(&self, name: &str) -> Option<r#enum::Decorator> {
        let enum_decorators = self.inner.enum_decorators.lock().unwrap();
        enum_decorators.get(name).cloned()
    }

    pub fn enum_decorator_at_path(&self, path: &Vec<&str>) -> Option<r#enum::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enum_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn enum_member_decorator(&self, name: &str) -> Option<r#enum::member::Decorator> {
        let enum_member_decorators = self.inner.enum_member_decorators.lock().unwrap();
        enum_member_decorators.get(name).cloned()
    }

    pub fn enum_member_decorator_at_path(&self, path: &Vec<&str>) -> Option<r#enum::member::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.enum_member_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_decorator(&self, name: &str) -> Option<interface::Decorator> {
        let interface_decorators = self.inner.interface_decorators.lock().unwrap();
        interface_decorators.get(name).cloned()
    }

    pub fn interface_decorator_at_path(&self, path: &Vec<&str>) -> Option<interface::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn interface_field_decorator(&self, name: &str) -> Option<interface::field::Decorator> {
        let interface_field_decorators = self.inner.interface_field_decorators.lock().unwrap();
        interface_field_decorators.get(name).cloned()
    }

    pub fn interface_field_decorator_at_path(&self, path: &Vec<&str>) -> Option<interface::field::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface_field_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn handler_decorator(&self, name: &str) -> Option<handler::Decorator> {
        let handler_decorators = self.inner.handler_decorators.lock().unwrap();
        handler_decorators.get(name).cloned()
    }

    pub fn handler_decorator_at_path(&self, path: &Vec<&str>) -> Option<handler::Decorator> {
        let decorator_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.handler_decorator(decorator_name)
        } else {
            None
        }
    }

    pub fn pipeline_item(&self, name: &str) -> Option<pipeline::Item> {
        let pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.get(name).cloned()
    }

    pub fn pipeline_item_at_path(&self, path: &Vec<&str>) -> Option<pipeline::Item> {
        let pipeline_item_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.pipeline_item(pipeline_item_name)
        } else {
            None
        }
    }

    pub fn r#struct(&self, name: &str) -> Option<Struct> {
        let structs = self.inner.structs.lock().unwrap();
        structs.get(name).cloned()
    }

    pub fn struct_at_path(&self, path: &Vec<&str>) -> Option<Struct> {
        let struct_name = *path.last().unwrap();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.r#struct(struct_name)
        } else {
            None
        }
    }

    pub fn r#enum(&self, name: &str) -> Option<Enum> {
        let enums = self.inner.enums.lock().unwrap();
        enums.get(name).cloned()
    }

    pub fn enum_at_path(&self, path: &Vec<&str>) -> Option<Enum> {
        let enum_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.r#enum(enum_name)
        } else {
            None
        }
    }

    pub fn model(&self, name: &str) -> Option<Model> {
        let models = self.inner.models.lock().unwrap();
        models.get(name).cloned()
    }

    pub fn model_at_path(&self, path: &Vec<&str>) -> Option<Model> {
        let model_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.model(model_name)
        } else {
            None
        }
    }

    pub fn interface(&self, name: &str) -> Option<Interface> {
        let interfaces = self.inner.interfaces.lock().unwrap();
        interfaces.get(name).cloned()
    }

    pub fn interface_at_path(&self, path: &Vec<&str>) -> Option<Interface> {
        let interface_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.interface(interface_name)
        } else {
            None
        }
    }

    pub fn middleware(&self, name: &str) -> Option<middleware::Definition> {
        let middlewares = self.inner.middlewares.lock().unwrap();
        middlewares.get(name).cloned()
    }

    pub fn middleware_at_path(&self, path: &Vec<&str>) -> Option<middleware::Definition> {
        let middleware_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        if let Some(ns) = self.namespace_at_path(&namespace_path) {
            ns.middleware(middleware_name)
        } else {
            None
        }
    }

    pub fn handler_template(&self, name: &str) -> Option<Handler> {
        let handler_templates = self.inner.handler_templates.lock().unwrap();
        handler_templates.get(name).cloned()
    }

    pub fn handler_template_at_path(&self, path: &Vec<&str>) -> Option<Handler> {
        let handler_name = path.last().unwrap().deref();
        if path.len() == 1 {
            self.handler_template(handler_name)
        } else {
            // try find a namespace first
            let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.handler_template(handler_name)
            } else {
                None
            }
        }
    }

    pub fn handler(&self, name: &str) -> Option<Handler> {
        let handlers = self.inner.handlers.lock().unwrap();
        handlers.get(name).cloned()
    }

    pub fn handler_group(&self, name: &str) -> Option<handler::Group> {
        let handler_groups = self.inner.handler_groups.lock().unwrap();
        handler_groups.get(name).cloned()
    }

    pub fn model_handler_group(&self, name: &str) -> Option<handler::Group> {
        let model_handler_groups = self.inner.model_handler_groups.lock().unwrap();
        model_handler_groups.get(name).cloned()
    }

    pub fn handler_at_path(&self, path: &Vec<&str>) -> Option<Handler> {
        let handler_name = path.last().unwrap().deref();
        if path.len() == 1 {
            self.handler(handler_name)
        } else {
            // try find a namespace first
            let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
            if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                dest_namespace.handler(handler_name)
            } else {
                // try find in group
                let handler_name = path.last().unwrap().deref();
                let group_name = path.get(path.len() - 2).unwrap().deref();
                let namespace_path: Vec<&str> = path.into_iter().rev().skip(2).rev().map(|i| *i).collect();
                if let Some(dest_namespace) = self.namespace_at_path(&namespace_path) {
                    if let Some(group) = dest_namespace.handler_group(group_name) {
                        group.handler(handler_name)
                    } else if let Some(group) = dest_namespace.model_handler_group(group_name) {
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

    pub fn replace_handler_template_at_path(&self, path: &Vec<&str>, handler: Handler) {
        let handler_name = path.last().unwrap().deref();
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(1).rev().map(|i| *i).collect();
        let dest_namespace = self.namespace_or_create_at_path(&namespace_path);
        dest_namespace.insert_handler(handler_name, handler);
    }

    pub fn replace_handler_at_path(&self, path: &Vec<&str>, handler: Handler, inside_group: bool) {
        let handler_name = path.last().unwrap().deref();
        let group_name = if inside_group {
            Some(path.get(path.len() - 2).unwrap().deref())
        } else {
            None
        };
        let namespace_path: Vec<&str> = path.into_iter().rev().skip(if inside_group { 2 } else { 1 }).rev().map(|i| *i).collect();
        let dest_namespace = self.namespace_or_create_at_path(&namespace_path);
        if let Some(group_name) = group_name {
            if let Some(group) = dest_namespace.handler_group(group_name) {
                group.insert_handler(handler_name, handler);
            } else if let Some(group) = dest_namespace.model_handler_group(group_name) {
                group.insert_handler(handler_name, handler);
            } else {
                dest_namespace.define_model_handler_group(group_name, |f| { });
                if let Some(group) = dest_namespace.model_handler_group(group_name) {
                    group.insert_handler(handler_name, handler);
                }
            }
        } else {
            dest_namespace.insert_handler(handler_name, handler);
        }
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
    pub fn opposite_relation(&self, relation: &Relation) -> (Model, Option<&Relation>) {
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
    pub fn through_relation(&self, relation: &Relation) -> (Model, &Relation) {
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
    pub fn through_opposite_relation(&self, relation: &Relation) -> (Model, &Relation) {
        let through_model = self.model_at_path(&relation.through_path().unwrap()).unwrap();
        let through_foreign_relation = through_model.relation(relation.foreign.as_ref().unwrap()).unwrap();
        (through_model, through_foreign_relation)
    }

    /// Get relations of model defined by related model
    pub fn model_opposite_relations(&self, model: &Model) -> Vec<(Model, &Relation)> {
        let model_opposite_relations_map = self.inner.model_opposite_relations_map.lock().unwrap();
        let result = model_opposite_relations_map.get(&model.path).unwrap();
        result.iter().map(|result| {
            let model = self.model_at_path(&result.0.iter().map(AsRef::as_ref).collect()).unwrap();
            let relation = model.relation(result.1.as_str()).unwrap();
            (model, relation)
        }).collect()
    }
}