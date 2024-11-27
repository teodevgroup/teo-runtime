use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use educe::Educe;
use maplit::btreemap;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::ast::middleware::MiddlewareType;
use teo_parser::r#type::Type;
use teo_result::{Error, Result};
use crate::interface::Interface;
use crate::{handler, interface, middleware, model, pipeline, r#enum, request, Value};
use crate::app::data::AppData;
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
use hyper::Method;
use crate::middleware::middleware::{empty_middleware, Middleware};
use crate::model::{Model, Relation};
use crate::namespace::Namespace;
use crate::pipeline::item::Call;
use crate::pipeline::item::item_impl::ItemImpl;
use crate::pipeline::item::templates::callback::{Callback, CallbackResult};
use crate::pipeline::item::templates::compare::Compare;
use crate::pipeline::item::templates::transformer::{TransformerResult, Transformer};
use crate::pipeline::item::templates::validator::{Validator, ValidatorResult};
use crate::r#enum::Enum;
use crate::r#struct::Struct;
use crate::stdlib::load::load;
use crate::utils::next_path;

#[derive(Clone, Debug)]
pub struct Builder {
    inner: Arc<Inner>
}

#[derive(Educe)]
#[educe(Debug)]
struct Inner {
    pub path: Vec<String>,
    pub namespaces: Arc<Mutex<BTreeMap<String, Builder>>>,
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
    pub request_middlewares: Arc<Mutex<BTreeMap<String, middleware::Definition>>>,
    pub handler_middlewares: Arc<Mutex<BTreeMap<String, middleware::Definition>>>,
    pub handlers: Arc<Mutex<BTreeMap<String, Handler>>>,
    pub handler_templates: Arc<Mutex<BTreeMap<String, Handler>>>,
    pub model_handler_groups: Arc<Mutex<BTreeMap<String, handler::group::Builder>>>,
    pub handler_groups: Arc<Mutex<BTreeMap<String, handler::group::Builder>>>,
    pub server: Arc<Mutex<Option<Server>>>,
    pub connector: Arc<Mutex<Option<Connector>>>,
    pub clients: Arc<Mutex<BTreeMap<String, Client>>>,
    pub entities: Arc<Mutex<BTreeMap<String, Entity>>>,
    pub debug: Arc<Mutex<Option<Debug>>>,
    pub admin: Arc<Mutex<Option<Admin>>>,
    pub request_middlewares_block: Arc<Mutex<Option<middleware::Block>>>,
    pub handler_middlewares_block: Arc<Mutex<Option<middleware::Block>>>,
    pub database: Arc<Mutex<Option<Database>>>,
    pub connector_reference: Arc<Mutex<Option<Vec<String>>>>,
    pub connection: Arc<Mutex<Option<Arc<dyn Connection>>>>,
    pub model_opposite_relations_map: Arc<Mutex<BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>>>,
    pub handler_map: Arc<Mutex<handler::Map>>,
    #[educe(Debug(ignore))]
    pub handler_middleware_stack: Arc<Mutex<&'static dyn Middleware>>,
    #[educe(Debug(ignore))]
    pub request_middleware_stack: Arc<Mutex<&'static dyn Middleware>>,
    pub app_data: AppData,
}

impl Builder {

    fn new(path: Vec<String>, app_data: AppData) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
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
                handler_middlewares: Arc::new(Mutex::new(Default::default())),
                request_middlewares: Arc::new(Mutex::new(Default::default())),
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
                request_middlewares_block: Arc::new(Mutex::new(None)),
                handler_middlewares_block: Arc::new(Mutex::new(None)),
                database: Arc::new(Mutex::new(None)),
                connector_reference: Arc::new(Mutex::new(None)),
                connection: Arc::new(Mutex::new(None)),
                model_opposite_relations_map: Arc::new(Mutex::new(Default::default())),
                handler_map: Arc::new(Mutex::new(handler::Map::new())),
                handler_middleware_stack: Arc::new(Mutex::new(empty_middleware())),
                request_middleware_stack: Arc::new(Mutex::new(empty_middleware())),
                app_data
            })
        }
    }

    pub fn main(app_data: AppData) -> Self {
        Self::new(vec![], app_data)
    }

    pub fn load_standard_library(&self) -> Result<()> {
        if self.inner.path.is_empty() {
            Err(Error::new("Standard library can only be loaded on main namespace"))?
        }
        load(self);
        Ok(())
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn is_main(&self) -> bool {
        self.path().is_empty()
    }

    pub fn is_std(&self) -> bool {
        self.path().len() == 0 && self.path().first().unwrap().as_str() == "std"
    }

    pub fn set_server(&self, server: Option<Server>) {
        *self.inner.server.lock().unwrap() = server;
    }

    pub fn set_connector(&self, connector: Option<Connector>) {
        *self.inner.connector.lock().unwrap() = connector;
    }

    pub fn connector(&self) -> Option<Connector> {
        self.inner.connector.lock().unwrap().clone()
    }

    pub fn set_debug(&self, debug: Option<Debug>) {
        *self.inner.debug.lock().unwrap() = debug;
    }

    pub fn insert_entity(&self, name: String, entity: Entity) {
        let mut entities = self.inner.entities.lock().unwrap();
        entities.insert(name, entity);
    }

    pub fn insert_client(&self, name: String, client: Client) {
        let mut clients = self.inner.clients.lock().unwrap();
        clients.insert(name, client);
    }

    pub fn set_admin(&self, admin: Option<Admin>) {
        *self.inner.admin.lock().unwrap() = admin;
    }

    pub fn set_database(&self, database: Option<Database>) {
        *self.inner.database.lock().unwrap() = database;
    }

    pub fn database(&self) -> Option<Database> {
        *self.inner.database.lock().unwrap()
    }

    pub fn set_connector_reference(&self, connector_reference: Option<Vec<String>>) {
        *self.inner.connector_reference.lock().unwrap() = connector_reference;
    }

    pub fn connector_reference(&self) -> Option<Vec<String>> {
        self.inner.connector_reference.lock().unwrap().clone()
    }

    pub fn request_middlewares_block(&self) -> Option<middleware::Block> {
        self.inner.request_middlewares_block.lock().unwrap().clone()
    }

    pub fn handler_middlewares_block(&self) -> Option<middleware::Block> {
        self.inner.handler_middlewares_block.lock().unwrap().clone()
    }

    pub fn insert_enum(&self, name: String, r#enum: Enum) {
        let mut enums = self.inner.enums.lock().unwrap();
        enums.insert(name, r#enum);
    }

    pub fn insert_model(&self, name: String, model: Model) {
        let mut models = self.inner.models.lock().unwrap();
        models.insert(name, model);
    }

    pub fn insert_interface(&self, name: String, interface: Interface) {
        let mut interfaces = self.inner.interfaces.lock().unwrap();
        interfaces.insert(name, interface);
    }

    pub fn namespaces(&self) -> BTreeMap<String, Builder> {
        self.inner.namespaces.lock().unwrap().clone()
    }

    pub fn child_namespace(&self, name: &str) -> Option<Builder> {
        let namespaces = self.inner.namespaces.lock().unwrap();
        if let Some(namespace) = namespaces.get(name) {
            Some(namespace.clone())
        } else {
            None
        }
    }

    pub fn child_namespace_or_create(&self, name: &str) -> Builder {
        let mut namespaces = self.inner.namespaces.lock().unwrap();
        if !namespaces.contains_key(name) {
            namespaces.insert(name.to_owned(), Builder::new(next_path(self.path(), name), self.app_data().clone()));
        }
        namespaces.get(name).unwrap().clone()
    }

    pub fn descendant_namespace_at_path(&self, path: &Vec<String>) -> Option<Builder> {
        let mut current = Some(self.clone());
        for item in path {
            if current.is_none() {
                return None;
            }
            current = current.unwrap().child_namespace(item);
        }
        current
    }

    pub fn descendant_namespace_or_create_at_path(&self, path: &Vec<String>) -> Builder {
        let mut current = self.clone();
        for item in path {
            current = current.child_namespace_or_create(item)
        }
        current
    }

    pub fn define_model_decorator<F>(&self, name: &str, call: F) where F: Fn(Arguments, &model::Builder) -> Result<()> + 'static + Send + Sync {
        let mut model_decorators = self.inner.model_decorators.lock().unwrap();
        model_decorators.insert(name.to_owned(), model::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_model_field_decorator(&self, name: &str, call: impl Fn(Arguments, &model::field::Builder) -> Result<()> + 'static + Send + Sync) {
        let mut model_field_decorators = self.inner.model_field_decorators.lock().unwrap();
        model_field_decorators.insert(name.to_owned(), model::field::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_model_relation_decorator(&self, name: &str, call: impl Fn(Arguments, &model::relation::Builder) -> Result<()> + 'static + Send + Sync) {
        let mut model_relation_decorators = self.inner.model_relation_decorators.lock().unwrap();
        model_relation_decorators.insert(name.to_owned(), model::relation::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_model_property_decorator(&self, name: &str, call: impl Fn(Arguments, &model::property::Builder) -> Result<()> + 'static + Send + Sync) {
        let mut model_property_decorators = self.inner.model_property_decorators.lock().unwrap();
        model_property_decorators.insert(name.to_owned(), model::property::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_enum_decorator(&self, name: &str, call: impl Fn(Arguments, &r#enum::Builder) -> Result<()> + 'static + Send + Sync) {
        let mut enum_decorators = self.inner.enum_decorators.lock().unwrap();
        enum_decorators.insert(name.to_owned(), r#enum::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_enum_member_decorator(&self, name: &str, call: impl Fn(Arguments, &r#enum::member::Builder) -> Result<()> + 'static + Send + Sync) {
        let mut enum_member_decorators = self.inner.enum_member_decorators.lock().unwrap();
        enum_member_decorators.insert(name.to_owned(), r#enum::member::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_interface_decorator<F>(&self, name: &str, call: F) where F: Fn(Arguments, &interface::Builder) -> Result<()> + 'static + Send + Sync {
        let mut interface_decorators = self.inner.interface_decorators.lock().unwrap();
        interface_decorators.insert(name.to_owned(), interface::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_handler_decorator(&self, name: &str, call: impl Fn(Arguments, &handler::Builder) -> Result<()> + 'static + Send + Sync) {
        let mut handler_decorators = self.inner.handler_decorators.lock().unwrap();
        handler_decorators.insert(name.to_owned(), handler::Decorator::new(next_path(self.path(), name), call));
    }

    pub fn define_pipeline_item<C, F>(&self, name: &str, creator: C) where
        C: Fn(Arguments) -> Result<F> + Clone + 'static,
        F: Call + 'static {
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item::new(next_path(self.path(), name), Arc::new(move |args| {
            let f = creator(args)?;
            Ok(ItemImpl::new(f))
        }), self.app_data().clone()));
    }

    pub fn define_transform_pipeline_item<C, A, O, F, R>(&self, name: &str, creator: C) where
        C: Fn(Arguments) -> Result<F> + Clone + 'static,
        A: Send + Sync + 'static,
        O: Into<Value> + Send + Sync + 'static,
        R: Into<TransformerResult<O>> + Send + Sync + 'static,
        F: Transformer<A, O, R> + 'static {
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item::new(next_path(self.path(), name), Arc::new(move |args: Arguments| {
            let creator = creator.clone();
            let transformer = creator(args)?;
            Ok(ItemImpl::new(move |ctx: pipeline::Ctx| {
                let transformer = transformer.clone();
                async move {
                    let transform_result: TransformerResult<O> = transformer.call(ctx).await.into();
                    match transform_result {
                        TransformerResult::Object(t) => Ok(t.into()),
                        TransformerResult::Result(r) => match r {
                            Ok(t) => Ok(t.into()),
                            Err(e) => Err(e.into()),
                        }
                    }
                }
            }))
        }), self.app_data().clone()));
    }

    pub fn define_validator_pipeline_item<C, T, F, O>(&self, name: &str, creator: C) where
        C: Fn(Arguments) -> Result<F> + Clone + 'static,
        T: Send + Sync + 'static,
        F: Validator<T, O> + 'static,
        O: Into<ValidatorResult> + Send + Sync + 'static {
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item::new(next_path(self.path(), name), Arc::new(move |args: Arguments| {
            let creator = creator.clone();
            let validator = creator(args)?;
            Ok(ItemImpl::new(move |ctx: pipeline::Ctx| {
                let validator = validator.clone();
                async move {
                    let ctx_value = ctx.value().clone();
                    let validate_result: ValidatorResult = validator.call(ctx).await.into();
                    match validate_result {
                        ValidatorResult::Validity(validity) => if validity.is_valid() {
                            Ok(ctx_value)
                        } else if let Some(reason) = validity.invalid_reason() {
                            Err(Error::new(reason))
                        } else {
                            Err(Error::new("value is invalid"))
                        },
                        ValidatorResult::Result(result) => match result {
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
                }
            }))
        }), self.app_data().clone()));
    }

    pub fn define_callback_pipeline_item<C, T, F, O>(&self, name: &str, creator: C) where
        C: Fn(Arguments) -> Result<F> + Clone + 'static,
        T: Send + Sync + 'static,
        F: Callback<T, O> + 'static,
        O: Into<CallbackResult> + Send + Sync + 'static {
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item::new(next_path(self.path(), name), Arc::new(move |args: Arguments| {
            let creator = creator.clone();
            let callback = creator(args)?;
            Ok(ItemImpl::new(move |ctx: pipeline::Ctx| {
                let callback = callback.clone();
                async move {
                    let ctx_value = ctx.value().clone();
                    let callback_result: CallbackResult = callback.call(ctx).await.into();
                    match callback_result {
                        CallbackResult::Result(t) => match t {
                            Ok(_) => Ok(ctx_value),
                            Err(err) => Err(err),
                        },
                    }
                }
            }))
        }), self.app_data().clone()));
    }

    pub fn define_compare_pipeline_item<C, T, O, F, E>(&self, name: &str, creator: C) where
        C: Fn(Arguments) -> Result<F> + Clone + 'static,
        T: Send + Sync + 'static,
        O: Into<ValidatorResult> + Send + Sync + 'static,
        E: Into<Error> + std::error::Error,
        F: Compare<T, O, E> + 'static {
        let mut pipeline_items = self.inner.pipeline_items.lock().unwrap();
        pipeline_items.insert(name.to_owned(), pipeline::Item::new(next_path(self.path(), name), Arc::new(move |args: Arguments| {
            let creator = creator.clone();
            let compare = creator(args)?;
            Ok(ItemImpl::new(move |ctx: pipeline::Ctx| {
                let compare = compare.clone();
                async move {
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
                    let validate_result: ValidatorResult = compare.call(previous_value, current_value, ctx).await.into();
                    match validate_result {
                        ValidatorResult::Validity(validity) => if validity.is_valid() {
                            Ok(ctx_value)
                        } else if let Some(reason) = validity.invalid_reason() {
                            Err(Error::new(reason))
                        } else {
                            Err(Error::new("value is invalid"))
                        },
                        ValidatorResult::Result(result) => match result {
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
                }
            }))
        }), self.app_data().clone()));
    }

    pub fn define_request_middleware<T>(&self, name: &str, call: T) where T: middleware::creator::Creator + 'static {
        let mut middlewares = self.inner.request_middlewares.lock().unwrap();
        middlewares.insert(name.to_owned(), middleware::Definition::new(next_path(self.path(), name), Arc::new(call), self.app_data().clone()));
    }

    pub fn define_handler_middleware<T>(&self, name: &str, call: T) where T: middleware::creator::Creator + 'static {
        let mut middlewares = self.inner.handler_middlewares.lock().unwrap();
        middlewares.insert(name.to_owned(), middleware::Definition::new(next_path(self.path(), name), Arc::new(call), self.app_data().clone()));
    }

    pub fn define_model_handler_group<T>(&self, name: &str, builder: T) where T: Fn(&handler::group::Builder) {
        let handler_group_builder = handler::group::Builder::new(next_path(self.path(), name), self.app_data().clone());
        builder(&handler_group_builder);
        let mut model_handler_groups = self.inner.model_handler_groups.lock().unwrap();
        model_handler_groups.insert(name.to_owned(), handler_group_builder);
    }

    pub fn insert_handler(&self, name: &str, handler: Handler) {
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }

    pub fn insert_handler_template(&self, name: &str, handler_template: Handler) {
        let mut handler_templates = self.inner.handler_templates.lock().unwrap();
        handler_templates.insert(name.to_owned(), handler_template);
    }

    pub fn define_handler<T, F>(&self, name: &str, call: F) where T: 'static, for<'a> F: 'static + HandlerCtxArgument<'a, T> {
        let wrapped_call: &'static F = &*Box::leak(Box::new(call));
        let builder = handler::Builder::new(
            next_path(self.path(), name),
            self.inner.path.clone(),
            Type::Undetermined,
            Type::Undetermined,
            false,
            HandlerInputFormat::Json,
            Box::leak(Box::new(move |request: request::Request| async move {
                wrapped_call.call(&request).await
            })),
            self.app_data().clone()
        );
        builder.set_method(Method::POST);
        builder.set_interface(None);
        builder.set_url(None);
        let handler = builder.build();
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }

    pub fn define_handler_template<T, F>(&self, name: &str, call: F) where T: 'static, for<'a> F: 'static + HandlerCtxArgument<'a, T> {
        let wrapped_call: &'static F = &*Box::leak(Box::new(call));
        let builder = handler::Builder::new(
            next_path(self.path(), name),
            self.inner.path.clone(),
            Type::Undetermined,
            Type::Undetermined,
            false,
            HandlerInputFormat::Json,
            Box::leak(Box::new(move |request: request::Request| async move {
                wrapped_call.call(&request).await
            })),
            self.app_data().clone()
        );
        builder.set_method(Method::POST);
        builder.set_interface(None);
        builder.set_url(None);
        let handler = builder.build();
        let mut handler_templates = self.inner.handler_templates.lock().unwrap();
        handler_templates.insert(name.to_owned(), handler);
    }

    pub fn define_handler_group<T>(&self, name: &str, builder: T) where T: Fn(&handler::group::Builder) {
        let handler_group_builder = handler::group::Builder::new(next_path(self.path(), name), self.app_data().clone());
        builder(&handler_group_builder);
        let mut handler_groups = self.inner.handler_groups.lock().unwrap();
        handler_groups.insert(name.to_owned(), handler_group_builder);
    }

    pub fn define_struct<T>(&self, name: &str, builder: T) where T: Fn(&'static Vec<String>, &mut Struct) {
        let path = Box::leak(Box::new(next_path(self.path(), name))) as &'static Vec<String>;
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let decorator_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let pipeline_item_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let enum_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
            ns.r#enum(enum_name)
        } else {
            None
        }
    }

    pub fn models(&self) -> BTreeMap<String, Model> {
        self.inner.models.lock().unwrap().clone()
    }

    pub fn model(&self, name: &str) -> Option<Model> {
        let models = self.inner.models.lock().unwrap();
        models.get(name).cloned()
    }

    pub fn model_at_path(&self, path: &Vec<String>) -> Option<Model> {
        let model_name = path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
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
        let interface_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
            ns.interface(interface_name)
        } else {
            None
        }
    }

    pub fn handler_middleware(&self, name: &str) -> Option<middleware::Definition> {
        let middlewares = self.inner.handler_middlewares.lock().unwrap();
        middlewares.get(name).cloned()
    }

    pub fn request_middleware(&self, name: &str) -> Option<middleware::Definition> {
        let middlewares = self.inner.request_middlewares.lock().unwrap();
        middlewares.get(name).cloned()
    }

    pub fn middleware_at_path_with_type(&self, path: &Vec<&str>, middleware_type: MiddlewareType) -> Option<middleware::Definition> {
        match middleware_type {
            MiddlewareType::HandlerMiddleware => self.handler_middleware_at_path(path),
            MiddlewareType::RequestMiddleware => self.request_middleware_at_path(path),
        }
    }

    pub fn handler_middleware_at_path(&self, path: &Vec<&str>) -> Option<middleware::Definition> {
        let middleware_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
            ns.handler_middleware(middleware_name)
        } else {
            None
        }
    }

    pub fn request_middleware_at_path(&self, path: &Vec<&str>) -> Option<middleware::Definition> {
        let middleware_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        if let Some(ns) = self.descendant_namespace_at_path(&namespace_path) {
            ns.request_middleware(middleware_name)
        } else {
            None
        }
    }

    pub fn handler_template(&self, name: &str) -> Option<Handler> {
        let handler_templates = self.inner.handler_templates.lock().unwrap();
        handler_templates.get(name).cloned()
    }

    pub fn handler_template_at_path(&self, path: &Vec<String>) -> Option<Handler> {
        let handler_name = path.last().unwrap();
        if path.len() == 1 {
            self.handler_template(handler_name)
        } else {
            // try finding a namespace first
            let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
            if let Some(dest_namespace) = self.descendant_namespace_at_path(&namespace_path) {
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

    pub fn handler_group(&self, name: &str) -> Option<handler::group::Builder> {
        let handler_groups = self.inner.handler_groups.lock().unwrap();
        handler_groups.get(name).cloned()
    }

    pub fn handler_group_or_create(&self, name: &str) -> handler::group::Builder {
        if let Some(handler_group) = self.handler_group(name) {
            handler_group
        } else {
            let mut handler_groups = self.inner.handler_groups.lock().unwrap();
            handler_groups.insert(name.to_owned(), handler::group::Builder::new(next_path(self.path(), name), self.app_data().clone()));
            handler_groups.get(name).unwrap().clone()
        }
    }

    pub fn model_handler_group(&self, name: &str) -> Option<handler::group::Builder> {
        let model_handler_groups = self.inner.model_handler_groups.lock().unwrap();
        model_handler_groups.get(name).cloned()
    }

    pub fn model_handler_group_or_create(&self, name: &str) -> handler::group::Builder {
        if let Some(model_handler_group) = self.model_handler_group(name) {
            model_handler_group
        } else {
            let mut model_handler_groups = self.inner.model_handler_groups.lock().unwrap();
            model_handler_groups.insert(name.to_owned(), handler::group::Builder::new(next_path(self.path(), name), self.app_data().clone()));
            model_handler_groups.get(name).unwrap().clone()
        }
    }

    pub fn handler_at_path(&self, path: &Vec<&str>) -> Option<Handler> {
        let handler_name = *path.last().unwrap();
        if path.len() == 1 {
            self.handler(handler_name)
        } else {
            // try finding a namespace first
            let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
            if let Some(dest_namespace) = self.descendant_namespace_at_path(&namespace_path) {
                dest_namespace.handler(handler_name)
            } else {
                // try finding in group
                let handler_name = *path.last().unwrap();
                let group_name = path.get(path.len() - 2).unwrap().deref();
                let namespace_path: Vec<String> = path.into_iter().rev().skip(2).rev().map(|i| i.to_string()).collect();
                if let Some(dest_namespace) = self.descendant_namespace_at_path(&namespace_path) {
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
        let handler_name = *path.last().unwrap();
        let namespace_path: Vec<String> = path.into_iter().rev().skip(1).rev().map(|i| i.to_string()).collect();
        let dest_namespace = self.descendant_namespace_or_create_at_path(&namespace_path);
        dest_namespace.insert_handler_template(handler_name, handler);
    }

    pub fn replace_handler_at_path(&self, path: &Vec<&str>, handler: Handler, inside_group: bool) {
        let handler_name = *path.last().unwrap();
        let group_name = if inside_group {
            Some(*path.get(path.len() - 2).unwrap())
        } else {
            None
        };
        let namespace_path: Vec<String> = path.into_iter().rev().skip(if inside_group { 2 } else { 1 }).rev().map(|i| i.to_string()).collect();
        let dest_namespace = self.descendant_namespace_or_create_at_path(&namespace_path);
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

    pub fn handler_map(&self) -> Arc<Mutex<handler::Map>> {
        self.inner.handler_map.clone()
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
    pub fn opposite_relation(&self, relation: &Relation) -> (Model, Option<Relation>) {
        let opposite_model = self.model_at_path(&relation.model_path()).unwrap();
        let opposite_relation = opposite_model.relations().values().find(|r| r.fields() == relation.references() && r.references() == relation.fields());
        (opposite_model.clone(), opposite_relation.cloned())
    }

    /// Returns the through relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph. This relation must be a
    /// 'through' relation.
    ///
    /// # Return Value
    ///
    /// A tuple of through relation's model and through model's local relation.
    ///
    pub fn through_relation(&self, relation: &Relation) -> (Model, Relation) {
        let through_model = self.model_at_path(relation.through_path().unwrap()).unwrap();
        let through_local_relation = through_model.relation(relation.local().unwrap()).unwrap();
        (through_model.clone(), through_local_relation.clone())
    }

    /// Returns the through opposite relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph. This relation must be a
    /// 'through' relation.
    ///
    /// # Return Value
    ///
    /// A tuple of through relation's model and through model's foreign relation.
    ///
    pub fn through_opposite_relation(&self, relation: &Relation) -> (Model, Relation) {
        let through_model = self.model_at_path(relation.through_path().unwrap()).unwrap();
        let through_foreign_relation = through_model.relation(relation.foreign().unwrap()).unwrap();
        (through_model.clone(), through_foreign_relation.clone())
    }

    /// Get relations of model defined by related model
    pub fn model_opposite_relations(&self, model: &Model) -> Vec<(Model, Relation)> {
        let model_opposite_relations_map = self.inner.model_opposite_relations_map.lock().unwrap();
        let result = model_opposite_relations_map.get(model.path()).unwrap();
        result.iter().map(|result| {
            let model = self.model_at_path(&result.0).unwrap();
            let relation = model.relation(result.1.as_str()).unwrap();
            (model.clone(), relation.clone())
        }).collect()
    }

    pub fn set_model_opposite_relations_map(&self, map: BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>) {
        *self.inner.model_opposite_relations_map.lock().unwrap() = map;
    }

    pub fn set_handler_middlewares_block(&self, block: Option<middleware::Block>) {
        *self.inner.handler_middlewares_block.lock().unwrap() = block;
    }

    pub fn set_request_middlewares_block(&self, block: Option<middleware::Block>) {
        *self.inner.request_middlewares_block.lock().unwrap() = block;
    }

    pub fn handler_middleware_stack(&self) -> &'static dyn Middleware {
        *self.inner.handler_middleware_stack.lock().unwrap()
    }

    pub fn set_handler_middleware_stack(&self, stack: &'static dyn Middleware) {
        *self.inner.handler_middleware_stack.lock().unwrap() = stack;
    }

    pub fn request_middleware_stack(&self) -> &'static dyn Middleware {
        *self.inner.request_middleware_stack.lock().unwrap()
    }

    pub fn set_request_middleware_stack(&self, stack: &'static dyn Middleware) {
        *self.inner.request_middleware_stack.lock().unwrap() = stack;
    }

    pub fn app_data(&self) -> &AppData {
        &self.inner.app_data
    }

    pub fn build(&self) -> Namespace {
        Namespace {
            inner: Arc::new(super::namespace::Inner {
                path: self.inner.path.clone(),
                namespaces: self.namespaces().into_iter().map(|(k, n)| (k.to_string(), n.build())).collect(),
                structs: self.inner.structs.lock().unwrap().clone(),
                models: self.inner.models.lock().unwrap().clone(),
                enums: self.inner.enums.lock().unwrap().clone(),
                interfaces: self.inner.interfaces.lock().unwrap().clone(),
                model_decorators: self.inner.model_decorators.lock().unwrap().clone(),
                model_field_decorators: self.inner.model_field_decorators.lock().unwrap().clone(),
                model_relation_decorators: self.inner.model_relation_decorators.lock().unwrap().clone(),
                model_property_decorators: self.inner.model_property_decorators.lock().unwrap().clone(),
                enum_decorators: self.inner.enum_decorators.lock().unwrap().clone(),
                enum_member_decorators: self.inner.enum_member_decorators.lock().unwrap().clone(),
                interface_decorators: self.inner.interface_decorators.lock().unwrap().clone(),
                interface_field_decorators: self.inner.interface_field_decorators.lock().unwrap().clone(),
                handler_decorators: self.inner.handler_decorators.lock().unwrap().clone(),
                pipeline_items: self.inner.pipeline_items.lock().unwrap().clone(),
                handler_middlewares: self.inner.handler_middlewares.lock().unwrap().clone(),
                request_middlewares: self.inner.request_middlewares.lock().unwrap().clone(),
                handlers: self.inner.handlers.lock().unwrap().clone(),
                handler_templates: self.inner.handler_templates.lock().unwrap().clone(),
                model_handler_groups: self.inner.model_handler_groups.lock().unwrap().clone().into_iter().map(|(k, v)| (k.to_string(), v.build())).collect(),
                handler_groups: self.inner.handler_groups.lock().unwrap().clone().into_iter().map(|(k, v)| (k.to_string(), v.build())).collect(),
                server: self.inner.server.lock().unwrap().clone(),
                connector: self.inner.connector.lock().unwrap().clone(),
                clients: self.inner.clients.lock().unwrap().clone(),
                entities: self.inner.entities.lock().unwrap().clone(),
                debug: self.inner.debug.lock().unwrap().clone(),
                admin: self.inner.admin.lock().unwrap().clone(),
                handler_middlewares_block: self.inner.handler_middlewares_block.lock().unwrap().clone(),
                request_middlewares_block: self.inner.request_middlewares_block.lock().unwrap().clone(),
                database: self.inner.database.lock().unwrap().clone(),
                connector_reference: self.inner.connector_reference.lock().unwrap().clone(),
                connection: self.inner.connection.clone(),
                handler_middleware_stack: self.inner.handler_middleware_stack.lock().unwrap().clone(),
                request_middleware_stack: self.inner.request_middleware_stack.lock().unwrap().clone(),
                handler_map: self.inner.handler_map.lock().unwrap().clone(),
                model_opposite_relations_map: self.inner.model_opposite_relations_map.lock().unwrap().clone(),
                app_data: self.app_data().clone(),
            })
        }
    }
}

unsafe impl Send for Builder { }
unsafe impl Sync for Builder { }