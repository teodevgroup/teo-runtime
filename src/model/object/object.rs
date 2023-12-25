use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Serialize, Serializer};
use teo_teon::Value;
use teo_result::{Error, Result};
use tokio::sync::Mutex as TokioMutex;
use crate::action::Action;
use crate::connection::transaction;
use crate::model::{Field, Model};
use key_path::{path, KeyPath};
use crate::traits::named::Named;
use async_recursion::async_recursion;
use futures_util::StreamExt;
use indexmap::{IndexMap, indexmap};
use itertools::Itertools;
use teo_teon::teon;
use crate::action::action::*;
use crate::model::object::input::Input;
use crate::model::object::input::Input::{AtomicUpdater, SetValue};
use crate::model::relation::Relation;
use crate::{object, pipeline, request};
use crate::model::field::column_named::ColumnNamed;
use crate::model::field::is_optional::IsOptional;
use crate::model::relation::delete::Delete;
use crate::namespace::Namespace;
use crate::object::error_ext;
use crate::optionality::Optionality;
use crate::readwrite::write::Write;
use crate::utils::ContainsStr;

#[derive(Clone)]
pub struct Object {
    pub inner: Arc<ObjectInner>
}

impl Object {

    pub fn new(request_ctx: Option<request::Ctx>, transaction_ctx: transaction::Ctx, model: &'static Model, action: Action) -> Object {
        Object {
            inner: Arc::new(ObjectInner {
                request_ctx,
                transaction_ctx,
                model,
                action,
                is_initialized: AtomicBool::new(false),
                is_new: AtomicBool::new(true),
                is_modified: AtomicBool::new(false),
                is_partial: AtomicBool::new(false),
                is_deleted: AtomicBool::new(false),
                inside_before_save_callback: AtomicBool::new(false),
                inside_after_save_callback: AtomicBool::new(false),
                selected_fields: Arc::new(Mutex::new(Vec::new())),
                modified_fields: Arc::new(Mutex::new(BTreeSet::new())),
                previous_value_map: Arc::new(Mutex::new(BTreeMap::new())),
                value_map: Arc::new(Mutex::new(BTreeMap::new())),
                atomic_updater_map: Arc::new(Mutex::new(BTreeMap::new())),
                relation_query_map: Arc::new(Mutex::new(BTreeMap::new())),
                relation_mutation_map: Arc::new(TokioMutex::new(BTreeMap::new())),
                cached_property_map: Arc::new(Mutex::new(BTreeMap::new())),
                object_set_map: Arc::new(TokioMutex::new(BTreeMap::new())),
                object_set_many_map: Arc::new(TokioMutex::new(BTreeMap::new())),
                object_connect_map: Arc::new(TokioMutex::new(BTreeMap::new())),
                object_disconnect_map: Arc::new(TokioMutex::new(BTreeMap::new())),
                ignore_relation: Arc::new(Mutex::new(None)),
            })
        }
    }

    pub fn transaction_ctx(&self) -> transaction::Ctx {
        self.inner.transaction_ctx.clone()
    }

    pub fn request_ctx(&self) -> Option<request::Ctx> {
        self.inner.request_ctx.clone()
    }

    pub fn model(&self) -> &'static Model {
        self.inner.model
    }

    pub fn namespace(&self) -> &'static Namespace {
        self.inner.transaction_ctx.namespace()
    }

    fn pipeline_ctx_for_path_and_value(&self, path: KeyPath, value: Value) -> pipeline::Ctx {
        pipeline::Ctx::new(object::Object::from(value), self.clone(), path, self.action(), self.transaction_ctx(), self.request_ctx())
    }

    pub async fn set_teon(&self, value: &Value) -> Result<()> {
        self.set_teon_with_path_and_user_mode(value, &path![], true).await?;
        Ok(())
    }

    pub async fn update_teon(&self, value: &Value) -> Result<()> {
        check_user_json_keys(value.as_dictionary().unwrap(), &self.model().cache.input_keys.iter().map(|k| k.as_str()).collect(), self.model())?;
        for (key, value) in value.as_dictionary().unwrap() {
            if self.model().field(key).is_some() {
                self.set_value(key, value.clone())?;
            } else if self.model().property(key).is_some() {
                self.set_property(key, value).await?;
            }
        }
        self.inner.is_initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub async fn set_teon_with_path(&self, json_value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        self.set_teon_with_path_and_user_mode(json_value, path, false).await
    }

    pub async fn set_teon_with_path_and_user_mode(&self, value: &Value, path: &KeyPath, bypass_permission_check: bool) -> crate::path::Result<()> {
        let model = self.model();
        // permission
        if !bypass_permission_check {
            self.check_model_write_permission(path).await?;
        }
        // get value map
        let value_map = value.as_dictionary().unwrap();
        let value_map_keys: Vec<&str> = value_map.keys().map(|k| k.as_str()).collect();
        // check keys
        if bypass_permission_check {
            check_user_json_keys(value_map, &model.cache.input_keys.iter().map(|k| k.as_str()).collect(), model)?;
        }
        // find keys to iterate
        let initialized = self.inner.is_initialized.load(Ordering::SeqCst);
        let keys = if initialized {
            self.model().cache.all_keys.iter().filter(|k| value_map_keys.contains(&k.as_str())).map(|k| k.as_str()).collect::<Vec<&str>>()
        } else {
            self.model().cache.all_keys.iter().map(|k| k.as_str()).collect()
        };
        // assign values
        for key in keys {
            let path = path + key;
            if let Some(field) = self.model().field(key) {
                let need_to_trigger_default_value = if initialized { false } else {
                    !value_map_keys.contains(&key)
                };
                if need_to_trigger_default_value {
                    // apply default values
                    if let Some(argument) = &field.default {
                        if let Some(pipeline) = argument.as_pipeline() {
                            let ctx = self.pipeline_ctx_for_path_and_value(path.clone(), Value::Null);
                            let value: Value = ctx.run_pipeline_into_path_value_error(pipeline).await?.try_into()?;
                            self.set_value_to_value_map(key, value);
                        } else if let Some(value) = argument.as_teon() {
                            self.set_value_to_value_map(key, value.clone());
                        }
                    }
                } else {
                    if !bypass_permission_check {
                        self.check_field_write_permission(field, &path).await?;
                    }
                    // set_value_to_value_map
                    let value = value_map.get(key).unwrap();
                    match Input::decode_field(value) {
                        AtomicUpdater(updator) => self.set_value_to_atomic_updator_map(key, updator),
                        SetValue(value) => {
                            // on set pipeline
                            let ctx = self.pipeline_ctx_for_path_and_value(path.clone(), value);
                            let value: Value = ctx.run_pipeline_into_path_value_error(&field.on_set).await?.try_into()?;
                            self.check_write_rule(key, &value, &path).await?;
                            self.set_value_to_value_map(key, value.clone());
                        }
                    }
                }
            } else if let Some(_) = self.model().relation(key) {
                let manipulation = match value_map.get(&key.to_string()) {
                    Some(value) => value,
                    None => continue,
                };
                self.set_value_to_relation_manipulation_map(key, manipulation).await;
            } else if let Some(property) = self.model().property(key) {
                if value_map_keys.contains(&key) {
                    if let Some(setter) = property.setter.as_ref() {
                        let value = value_map.get(&key.to_string()).unwrap();
                        let input_result = Input::decode_field(value);
                        let value = match input_result {
                            SetValue(v) => v,
                            _ => return Err(error_ext::unexpected_input(path + key)),
                        };
                        let ctx = self.pipeline_ctx_for_path_and_value(path.clone(), value);
                        let _ = ctx.run_pipeline_into_path_value_error(setter).await?;
                    }
                }
            }
        };
        // set flag
        self.inner.is_initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn copied_value(&self) -> Value {
        let mut map = indexmap! {};
        for (k, v) in self.inner.value_map.lock().unwrap().iter() {
            let field = self.model().field(k).unwrap();
            if field.copy {
                map.insert(k.to_owned(), v.clone());
            }
        }
        Value::Dictionary(map)
    }

    async fn check_model_write_permission<'a>(&self, path: impl AsRef<KeyPath>) -> Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        ctx.run_pipeline_into_path_value_error(&self.model().can_mutate).await?;
        Ok(())
    }

    async fn check_model_read_permission<'a>(&self, path: impl AsRef<KeyPath>) -> Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        ctx.run_pipeline_into_path_value_error(&self.model().can_read).await?;
        Ok(())
    }

    async fn check_field_write_permission<'a>(&self, field: &Field, path: impl AsRef<KeyPath>) -> Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        ctx.run_pipeline_into_path_value_error(&field.can_mutate).await?;
        Ok(())
    }

    async fn check_field_read_permission<'a>(&self, field: &Field, path: impl AsRef<KeyPath>) -> Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        ctx.run_pipeline_into_path_value_error(&field.can_read).await?;
        Ok(())
    }

    fn record_previous_value_for_field_if_needed(&self, key: &str) {
        let field = self.model().field(key).unwrap();
        if !self.is_new() && field.previous.is_keep() {
            if self.inner.previous_value_map.lock().unwrap().get(field.name()).is_none() {
                self.inner.previous_value_map.lock().unwrap().insert(field.name().to_string(), self.get_value(field.name()).unwrap());
            }
        }
    }

    async fn check_write_rule(&self, key: impl AsRef<str>, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let field = self.model().field(key.as_ref()).unwrap();
        let is_new = self.is_new();
        let valid = match &field.write {
            Write::NoWrite => false,
            Write::Write => true,
            Write::WriteOnCreate => is_new,
            Write::WriteOnce => if is_new { true } else { self.get_value(key.as_ref()).unwrap().is_null() },
            Write::WriteNonNull => if is_new { true } else { !value.is_null() },
            Write::WriteIf(pipeline) => {
                let ctx = self.pipeline_ctx_for_path_and_value(path + key.as_ref(), value.clone());
                ctx.run_pipeline_into_path_value_error(pipeline).await.is_ok()
            }
        };
        if !valid {
            Err(crate::path::Error::value_error(path + key.as_ref(), "unexpected key"))
        } else {
            Ok(())
        }
    }

    fn set_modified_field(&self, key: &str) {
        if !self.is_new() {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.lock().unwrap().insert(key.to_string());
        }
    }

    fn set_value_to_atomic_updator_map(&self, key: &str, value: Value) {
        self.inner.atomic_updater_map.lock().unwrap().insert(key.to_string(), value);
        self.set_modified_field(key);
    }

    async fn set_value_to_relation_manipulation_map(&self, key: &str, value: &Value) {
        self.inner.relation_mutation_map.lock().await.insert(key.to_string(), value.clone());
        self.set_modified_field(key);
    }

    pub fn set(&self, key: impl AsRef<str>, value: impl Into<Value>) -> Result<()> {
        self.set_value(key, value.into())
    }

    pub fn set_value(&self, key: impl AsRef<str>, value: Value) -> Result<()> {
        let model_keys = &self.model().cache.save_keys_and_virtual_keys;
        if !model_keys.contains_str(key.as_ref()) {
            return Err(Error::new(format!("invalid key {}", key.as_ref())));
        }
        self.set_value_to_value_map(key.as_ref(), value);
        Ok(())
    }

    pub async fn set_property(&self, key: &str, value: impl Into<Value>) -> Result<()> {
        let property = self.model().property(key).unwrap();
        let setter = property.setter.as_ref().unwrap();
        let ctx = self.pipeline_ctx_for_path_and_value(path![key], value.into());
        ctx.run_pipeline(setter).await?;
        Ok(())
    }

    pub fn set_from_database_result_value(&self, value: &Value, select: Option<&Value>, include: Option<&Value>) {
        let model = self.model();
        for (k, v) in value.as_dictionary().unwrap() {
            if let Some(_) = model.field(k) {
                self.set_value_to_value_map(k, v.clone());
            } else if let Some(relation) = model.relation(k) {
                self.inner.relation_query_map.lock().unwrap().insert(k.to_owned(), vec![]);
                let include_arg = include.unwrap().get(k).unwrap();
                let inner_select = include_arg.as_dictionary().map(|m| m.get("select")).flatten();
                let inner_include = include_arg.as_dictionary().map(|m| m.get("include")).flatten();
                for v in v.as_array().unwrap() {
                    let action = FIND | (if relation.is_vec { MANY } else { SINGLE }) | NESTED ;
                    let object = self.transaction_ctx().new_object(self.namespace().model_at_path(&relation.model_path()).unwrap(), action, self.request_ctx()).unwrap();
                    object.set_from_database_result_value(v, inner_select, inner_include);
                    self.inner.relation_query_map.lock().unwrap().get_mut(k).unwrap().push(object);
                }
            } else if let Some(_property) = model.property(k) {
                self.inner.cached_property_map.lock().unwrap().insert(k.to_owned(), v.clone());
            }
        }
        self.set_select(select).unwrap();
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
    }

    fn set_value_to_value_map(&self, key: &str, value: Value) {
        // record previous value if needed
        self.record_previous_value_for_field_if_needed(key);

        if value.is_null() {
            self.inner.value_map.lock().unwrap().remove(key);
        } else {
            self.inner.value_map.lock().unwrap().insert(key.to_string(), value);
        }
        if !self.is_new() {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.lock().unwrap().insert(key.to_string());
            if let Some(properties) = self.model().cache.field_property_map.get(key) {
                for property in properties {
                    self.inner.modified_fields.lock().unwrap().insert(property.to_string());
                    self.inner.cached_property_map.lock().unwrap().remove(&property.to_string());
                }
            }
        }
    }

    pub fn get_query_relation_object(&self, key: impl AsRef<str>, path: &KeyPath) -> Result<Option<Object>> {
        let key = key.as_ref();
        let model_keys = &self.model().cache.all_keys;
        if !model_keys.contains_str(&key) {
            Err(error_ext::invalid_key_on_model(path.clone(), key, self.model()))?;
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => Ok(list.get(0).cloned()),
            None => Ok(None)
        }
    }

    pub fn get_mutation_relation_object(&self, key: impl AsRef<str>) -> Result<Option<Object>> {
        let key = key.as_ref();
        let model_keys = &self.model().cache.all_keys;
        if !model_keys.contains_str(&key) {
            Err(error_ext::invalid_key_on_model(Default::default(), key, self.model()))?;
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => Ok(list.get(0).cloned()),
            None => Ok(None)
        }
    }

    pub fn has_query_relation_fetched(&self, key: impl AsRef<str>) -> bool {
        self.inner.relation_query_map.lock().unwrap().contains_key(key.as_ref())
    }

    pub fn has_mutation_relation_fetched(&self, key: impl AsRef<str>) -> bool {
        self.inner.relation_query_map.lock().unwrap().contains_key(key.as_ref())
    }

    pub fn get_relation_vec(&self, key: impl AsRef<str>) -> Result<Vec<Object>> {
        let key = key.as_ref();
        let model_keys = &self.model().cache.all_keys;
        if !model_keys.contains_str(key) {
            return Err(error_ext::invalid_key_on_model(Default::default(), key, self.model()))?;
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => Ok(list.clone()),
            None => Ok(vec![]),
        }
    }

    pub async fn get_property<T>(&self, key: &str) -> Result<T> where T: TryFrom<Value, Error = Error> {
        Ok(self.get_property_value(key).await?.try_into()?)
    }

    pub async fn get_property_value(&self, key: &str) -> Result<Value> {
        let property = self.model().property(key.as_ref()).unwrap();
        if property.cached {
            if let Some(value) = self.inner.cached_property_map.lock().unwrap().get(key) {
                return Ok(value.clone());
            }
        }
        let getter = property.getter.as_ref().unwrap();
        let ctx = self.pipeline_ctx_for_path_and_value(path![key], Value::Null);
        let value: Value = ctx.run_pipeline(getter).await?.try_into()?;
        if property.cached {
            self.inner.cached_property_map.lock().unwrap().insert(key.to_string(), value.clone());
        }
        Ok(value)
    }

    pub fn get<T>(&self, key: impl AsRef<str>) -> Result<T> where T: TryFrom<Value, Error = Error> {
        match self.get_value(key) {
            Ok(optional_value) => {
                Ok(optional_value.try_into()?)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn get_previous_value(&self, key: impl AsRef<str>) -> Result<Value> {
        let key = key.as_ref();
        let model_keys = &self.model().cache.all_keys;
        if !model_keys.contains_str(key) {
            let model = self.model();
            Err(error_ext::invalid_key_on_model(Default::default(), key, model))?;
        }
        let map = self.inner.previous_value_map.lock().unwrap();
        match map.get(key) {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Null),
        }
    }

    fn get_value_map_value(&self, key: &str) -> Value {
        match self.inner.value_map.lock().unwrap().get(key) {
            Some(value) => value.clone(),
            None => Value::Null,
        }
    }

    pub fn get_value(&self, key: impl AsRef<str>) -> Result<Value> {
        let model_keys = &self.model().cache.all_keys;
        if !model_keys.contains_str(key.as_ref()) {
            Err(error_ext::invalid_key_on_model(KeyPath::default(), key.as_ref(), self.model()))?;
        }
        Ok(self.get_value_map_value(key.as_ref()))
    }

    pub fn get_atomic_updator(&self, key: &str) -> Option<Value> {
        self.inner.atomic_updater_map.lock().unwrap().get(key).cloned()
    }

    pub fn atomic_updators(&self) -> MutexGuard<BTreeMap<String, Value>> {
        self.inner.atomic_updater_map.lock().unwrap()
    }

    pub fn set_select(&self, select: Option<&Value>) -> Result<()> {
        if select.is_none() {
            return Ok(());
        }
        let mut true_list: Vec<&str> = vec![];
        let mut false_list: Vec<&str> = vec![];
        let map = select.unwrap().as_dictionary().unwrap();
        for (key, value) in map {
            let bool_value = value.as_bool().unwrap();
            if bool_value {
                true_list.push(key.as_str());
            } else {
                false_list.push(key.as_str());
            }
        }
        let true_empty = true_list.is_empty();
        let false_empty = false_list.is_empty();
        if true_empty && false_empty {
            // just do nothing
            return Ok(());
        } else if !false_empty {
            // all - false
            let mut result: Vec<String> = vec![];
            self.model().cache.all_keys.iter().for_each(|k| {
                if let Some(field) = self.model().field(k) {
                    if !false_list.contains(&&***&k) {
                        result.push(field.name.to_string());
                    }
                } else if let Some(property) = self.model().property(k) {
                    if !false_list.contains(&&***&k) {
                        result.push(property.name.to_string());
                    }
                }
            });
            *self.inner.selected_fields.lock().unwrap() = result;
            return Ok(());
        } else {
            // true
            let mut result: Vec<String> = vec![];
            self.model().cache.all_keys.iter().for_each(|k| {
                if let Some(field) = self.model().field(k) {
                    if true_list.contains(&k.as_str()) {
                        result.push(field.name.to_string());
                    }
                } else if let Some(property) = self.model().property(k) {
                    if true_list.contains(&k.as_str()) {
                        result.push(property.name.to_string());
                    }
                }
            });
            *self.inner.selected_fields.lock().unwrap() = result;
            return Ok(());
        }
    }

    #[async_recursion]
    pub async fn apply_on_save_pipeline_and_validate_required_fields(&self, path: &KeyPath, ignore_required_relation: bool) -> crate::path::Result<()> {
        // apply on save pipeline first
        let model_keys = &self.model().cache.save_keys;
        for key in model_keys {
            let field = self.model().field(key);
            if field.is_none() {
                continue;
            }
            let field = field.unwrap();
            if !field.on_save.is_empty() {
                let initial_value = match self.inner.value_map.lock().unwrap().deref().get(&key.to_string()) {
                    Some(value) => {
                        value.clone()
                    }
                    None => {
                        Value::Null
                    }
                };
                let ctx = self.pipeline_ctx_for_path_and_value(path + field.name(), initial_value);
                let value: Value = ctx.run_pipeline_into_path_value_error(&field.on_save).await?.try_into()?;
                self.inner.value_map.lock().unwrap().insert(key.to_string(), value);
                self.set_modified_field(key);
            }
        }
        // validate required fields
        for key in model_keys {
            if let Some(field) = self.model().field(key) {
                if field.auto || field.auto_increment || field.foreign_key {
                    continue
                }
                match &field.optionality {
                    Optionality::Optional => (),
                    Optionality::Required => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            return Err(error_ext::missing_required_input(path + key.as_str()));
                        }
                    }
                    Optionality::PresentWith(field_names) => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            for name in field_names {
                                let name = name.as_str();
                                let value_at_name = self.get_value(name).unwrap();
                                if !value_at_name.is_null() {
                                    return Err(error_ext::missing_required_input_with_type(path.clone(), key))
                                }
                            }
                        }
                    }
                    Optionality::PresentWithout(field_names) => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            for name in field_names {
                                let name = name.as_str();
                                let value_at_name = self.get_value(name).unwrap();
                                if !value_at_name.is_null() {
                                    break;
                                }
                                return Err(error_ext::missing_required_input_with_type(path.clone(), key));
                            }
                        }
                    }
                    Optionality::PresentIf(pipeline) => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            let ctx = self.pipeline_ctx_for_path_and_value(path + field.name(), Value::Null);
                            let invalid = ctx.run_pipeline_into_path_value_error(pipeline).await.is_err();
                            if invalid {
                                return Err(error_ext::missing_required_input(path + field.name()));
                            }
                        }
                    }
                }
            }
        }
        // validate required relations
        for key in &self.model().cache.relation_output_keys {
            if let Some(relation) = self.model().relation(key) {
                if let Some(ignore) = self.inner.ignore_relation.lock().unwrap().as_ref() {
                    if ignore.as_str() == relation.name() {
                        continue
                    }
                }
                if self.is_new() && relation.is_required() && !relation.is_vec {
                    if ignore_required_relation {
                        continue
                    }
                    // check whether foreign key is received or a value is provided
                    let map = self.inner.relation_mutation_map.lock().await;
                    if map.get(&key.to_string()).is_some() {
                        continue
                    }
                    for field_name in &relation.fields {
                        if self.get_value(field_name).unwrap().is_null() {
                            return Err(error_ext::missing_required_input(path + key.as_str()));
                        }
                    }
                    continue
                }
            }
        }
        Ok(())
    }

    pub fn clear_new_state(&self) {
        let is_new = self.is_new();
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
        // todo: set self as identity when identity
    }

    pub fn clear_state(&self) {
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
        *self.inner.modified_fields.lock().unwrap() = BTreeSet::new();
    }

    #[async_recursion]
    pub async fn delete_from_database(&self, path: &KeyPath) -> crate::path::Result<()> {
        let model = self.model();
        let namespace = self.namespace();
        // check deny first
        for (opposite_model, opposite_relation) in namespace.model_opposite_relations(model) {
            if opposite_relation.delete == Delete::Deny {
                let finder = self.intrinsic_where_unique_for_opposite_relation(opposite_relation);
                let count = self.transaction_ctx().count(opposite_model, &finder, path.clone()).await.unwrap();
                if count > 0 {
                    return Err(error_ext::deletion_denied(path.clone(), &format!("{}.{}", opposite_model.path().join("."), opposite_relation.name())));
                }
            }
        }
        // real delete
        self.transaction_ctx().transaction_for_model(self.model()).await.delete_object(self, path.clone()).await?;
        // nullify and cascade
        for (opposite_model, opposite_relation) in namespace.model_opposite_relations(model) {
            match opposite_relation.delete {
                Delete::NoAction => {} // do nothing
                Delete::Deny => {}, // done before
                Delete::Nullify => {
                    if !opposite_relation.has_foreign_key {
                        continue
                    }
                    let finder = self.intrinsic_where_unique_for_opposite_relation(opposite_relation);
                    self.transaction_ctx().batch(opposite_model, &finder, CODE_NAME | DISCONNECT | SINGLE, self.request_ctx(), path.clone(), |object| async move {
                        for key in &opposite_relation.fields {
                            object.set_value(key, Value::Null)?;
                        }
                        object.save_with_session_and_path( &path![]).await?;
                        Ok(())
                    }).await?;
                },
                Delete::Cascade => {
                    let finder = self.intrinsic_where_unique_for_opposite_relation(opposite_relation);
                    self.transaction_ctx().batch(opposite_model, &finder, CODE_NAME | DELETE | SINGLE, self.request_ctx(), path.clone(), |object| async move {
                        object.delete_from_database(path).await?;
                        Ok(())
                    }).await?;
                }
                Delete::Default => {
                    let finder = self.intrinsic_where_unique_for_opposite_relation(opposite_relation);
                    self.transaction_ctx().batch(opposite_model, &finder, CODE_NAME | DISCONNECT | SINGLE, self.request_ctx(), path.clone(), |object| async move {
                        for key in &opposite_relation.fields {
                            let field = opposite_model.field(key).unwrap();
                            if let Some(default) = &field.default {
                                if let Some(value) = default.as_teon() {
                                    object.set_value(key, value.clone())?;
                                } else if let Some(pipeline) = default.as_pipeline() {
                                    let pipeline_ctx = pipeline::Ctx::new(Value::Null.into(), object.clone(), path![], CODE_NAME | DISCONNECT | SINGLE, self.transaction_ctx(), self.request_ctx());
                                    let value_object = pipeline_ctx.run_pipeline(pipeline).await?;
                                    let value = value_object.as_teon().unwrap();
                                    object.set_value(key, value.clone())?;
                                }
                            } else {
                                Err(Error::new(format!("default value is not defined: {}.{}", opposite_model.path.join("."), key)))?;
                            }
                        }
                        Ok(())
                    }).await?;
                }
            }
        }
        Ok(())
    }

    #[async_recursion]
    async fn save_to_database(&self, path: &KeyPath) -> crate::path::Result<()> {
        self.transaction_ctx().transaction_for_model(self.model()).await.save_object(self, path.clone()).await?;
        self.clear_new_state();
        Ok(())
    }

    fn before_save_callback_check(&self, path: &KeyPath) -> crate::path::Result<()> {
        let inside_before_callback = self.inner.inside_before_save_callback.load(Ordering::SeqCst);
        if inside_before_callback {
            return Err(error_ext::invalid_operation(path.clone(), "save called inside before callback"));
        }
        Ok(())
    }

    pub async fn save_with_session_and_path(&self, path: &KeyPath) -> crate::path::Result<()> {
        self.save_with_session_and_path_and_ignore(path, false).await
    }

    #[async_recursion]
    pub async fn save_with_session_and_path_and_ignore(&self, path: &KeyPath, ignore_required_relation: bool) -> crate::path::Result<()> {
        // check if it's inside before callback
        self.before_save_callback_check(path)?;
        let is_new = self.is_new();
        // validate and save
        let is_modified = self.is_modified();
        if is_modified || is_new {
            // apply pipeline
            self.apply_on_save_pipeline_and_validate_required_fields(path, ignore_required_relation).await?;
            self.trigger_before_save_callbacks(path).await?;
            // perform relation manipulations (has foreign key)
            self.perform_relation_manipulations(|r| r.has_foreign_key, path).await?;
            self.save_to_database(path).await?;
        } else {
            // perform relation manipulations (has foreign key)
            self.perform_relation_manipulations(|r| r.has_foreign_key, path).await?;
        }
        // perform relation manipulations (doesn't have foreign key)
        self.perform_relation_manipulations(|r| !r.has_foreign_key, path).await?;
        // clear properties
        self.clear_state();
        if is_modified || is_new {
            self.trigger_after_save_callbacks(path).await?;
        }
        Ok(())
    }

    pub async fn save(&self) -> Result<()> {
        self.save_with_session_and_path(&path![]).await?;
        Ok(())
    }

    pub async fn save_for_seed_without_required_relation(&self) -> crate::path::Result<()> {
        self.save_with_session_and_path_and_ignore(&path![], true).await
    }

    async fn trigger_before_delete_callbacks<'a>(&self, path: impl AsRef<KeyPath>) -> crate::path::Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        let _ = ctx.run_pipeline_into_path_unauthorized_error(&self.model().before_delete).await?;
        Ok(())
    }

    async fn trigger_after_delete_callbacks<'a>(&self, path: impl AsRef<KeyPath>) -> crate::path::Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        let _ = ctx.run_pipeline_into_path_unauthorized_error(&self.model().after_delete).await?;
        Ok(())
    }

    async fn trigger_before_save_callbacks<'a>(&self, path: impl AsRef<KeyPath>) -> crate::path::Result<()> {
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        let _ = ctx.run_pipeline_into_path_unauthorized_error(&self.model().before_save).await?;
        Ok(())
    }

    async fn trigger_after_save_callbacks<'a>(&self, path: impl AsRef<KeyPath>) -> Result<()> {
        let inside_after_save_callback = self.inner.inside_after_save_callback.load(Ordering::SeqCst);
        if inside_after_save_callback {
            return Ok(());
        }
        self.inner.inside_after_save_callback.store(true, Ordering::SeqCst);
        let ctx = self.pipeline_ctx_for_path_and_value(path.as_ref().clone(), Value::Null);
        ctx.run_pipeline_into_path_unauthorized_error(&self.model().after_save).await?;
        self.inner.inside_after_save_callback.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub async fn delete(&self) -> Result<()> {
        self.trigger_before_delete_callbacks(path![]).await?;
        self.delete_from_database(&path![]).await?;
        Ok(())
    }

    pub async fn delete_internal<'a>(&self, path: impl AsRef<KeyPath>) -> crate::path::Result<()> {
        self.check_model_write_permission(path.as_ref()).await?;
        self.trigger_before_delete_callbacks(path.as_ref()).await?;
        self.delete_from_database(path.as_ref()).await?;
        self.trigger_after_delete_callbacks(path.as_ref()).await
    }

    pub async fn to_teon(&self) -> crate::path::Result<Value> {
        self.to_teon_internal(&path![]).await
    }

    #[async_recursion]
    pub async fn to_teon_internal<'a>(&self, path: &KeyPath) -> crate::path::Result<Value> {
        // check read permission
        self.check_model_read_permission(path.as_ref()).await?;
        // output
        let select_list = self.inner.selected_fields.lock().unwrap().clone();
        let select_filter = if select_list.is_empty() { false } else { true };
        let mut map: IndexMap<String, Value> = IndexMap::new();
        let keys = &self.model().cache.output_keys;
        for key in keys {
            if let Some(relation) = self.model().relation(key) {
                if self.has_query_relation_fetched(relation.name()) {
                    if !relation.is_vec {
                        let o = self.get_query_relation_object(key, &(path + key)).unwrap();
                        match o {
                            Some(o) => {
                                map.insert(key.to_string(), o.to_teon_internal(&(path.as_ref() + relation.name())).await.unwrap());
                            },
                            None => ()
                        };
                    } else {
                        let mut result_vec = vec![];
                        let vec = self.get_relation_vec(key).unwrap();
                        for (index, o) in vec.iter().enumerate() {
                            result_vec.push(o.to_teon_internal(&(path.as_ref() + relation.name() + index)).await?);
                        }
                        map.insert(key.to_string(), Value::Array(result_vec));
                    }
                }
            } else if (!select_filter) || (select_filter && select_list.contains(&key.to_string())) {
                if let Some(field) = self.model().field(key) {
                    let value = self.get_value(key).unwrap();
                    if self.check_field_read_permission(field, path.as_ref()).await.is_err() {
                        continue
                    }
                    let ctx = self.pipeline_ctx_for_path_and_value(path![key], value);
                    let value: Value = ctx.run_pipeline_into_path_value_error(&field.on_output).await?.try_into()?;
                    if !value.is_null() {
                        map.insert(key.to_string(), value);
                    }
                } else if let Some(property) = self.model().property(key) {
                    if property.cached && self.inner.cached_property_map.lock().unwrap().contains_key(&key.to_string()) {
                        let value = self.inner.cached_property_map.lock().unwrap().get(&key.to_string()).unwrap().clone();
                        if !value.is_null() {
                            map.insert(key.to_string(), value);
                        }
                    } else {
                        if let Some(getter) = &property.getter {
                            let ctx = self.pipeline_ctx_for_path_and_value(path![key], Value::Null);
                            let value: Value = ctx.run_pipeline_into_path_value_error(&getter).await?.try_into()?;
                            if !value.is_null() {
                                map.insert(key.to_string(), value);
                            }
                        }
                    }
                }
            }
        }
        return Ok(Value::Dictionary(map))
    }

    pub fn is_new(&self) -> bool {
        self.inner.is_new.load(Ordering::SeqCst)
    }

    pub fn is_modified(&self) -> bool {
        self.inner.is_modified.load(Ordering::SeqCst)
    }

    pub fn identifier(&self) -> Value {
        let model = self.model();
        let mut identifier: IndexMap<String, Value> = IndexMap::new();
        for item in model.primary_index().unwrap().items() {
            let val = self.get_value(&item.field).unwrap();
            identifier.insert(item.field.to_owned(), val);
        }
        Value::Dictionary(identifier)
    }

    pub fn previous_identifier(&self) -> Value {
        let model = self.model();
        let mut identifier: IndexMap<String, Value> = IndexMap::new();
        for item in model.primary_index().unwrap().items() {
            let modify_map = self.inner.modified_fields.lock().unwrap();
            let val = if modify_map.contains(&item.field) {
                if let Ok(val) = self.get_previous_value(&item.field) {
                    if val.is_null() {
                        self.get_value(&item.field).unwrap()
                    } else {
                        val
                    }
                } else {
                    self.get_value(&item.field).unwrap()
                }
            } else {
                self.get_value(&item.field).unwrap()
            };
            identifier.insert(item.field.to_owned(), val);
        }
        Value::Dictionary(identifier)
    }

    pub fn db_identifier(&self) -> Value {
        let model = self.model();
        let mut identifier: IndexMap<String, Value> = IndexMap::new();
        let modified_fields = self.inner.modified_fields.lock().unwrap();
        for item in model.primary_index().unwrap().items() {
            let val = if modified_fields.contains(&item.field) {
                self.get_previous_value(&item.field).unwrap()
            } else {
                self.get_value(&item.field).unwrap()
            };
            identifier.insert(self.model().field(&item.field).unwrap().column_name().to_owned(), val.clone());
        }
        Value::Dictionary(identifier)
    }

    async fn perform_relation_manipulations<F: Fn(&'static Relation) -> bool>(&self, f: F, path: &KeyPath) -> Result<()> {
        for relation in self.model().relations() {
            if f(relation) {
                let many = relation.is_vec;
                // programming code set
                if many {
                    let object_set_many_map = self.inner.object_set_many_map.lock().await;
                    if let Some(objects_to_set) = object_set_many_map.get(relation.name()) {
                        self.nested_set_many_relation_object_object(relation, objects_to_set, path).await?;
                    }
                } else {
                    let object_set_map = self.inner.object_set_map.lock().await;
                    if let Some(option) = object_set_map.get(relation.name()) {
                        // disconnect current
                        let value = self.intrinsic_where_unique_for_relation(relation);
                        self.nested_disconnect_relation_object(relation, &value, path).await?;
                        if let Some(new_object) = option {
                            // connect new
                            self.link_and_save_relation_object(relation, new_object, path).await?;
                        }
                    }
                }
                // programming code connections
                let object_connect_map = self.inner.object_connect_map.lock().await;
                if let Some(objects_to_connect) = object_connect_map.get(relation.name()) {
                    for object in objects_to_connect {
                        self.link_and_save_relation_object(relation, object, path).await?;
                    }
                }
                // programming code disconnections
                let object_disconnect_map = self.inner.object_disconnect_map.lock().await;
                if let Some(objects_to_disconnect) = object_disconnect_map.get(relation.name()) {
                    for object in objects_to_disconnect {
                        if relation.has_join_table() {
                            self.delete_join_object(object, relation, self.namespace().opposite_relation(relation).1.unwrap(), path).await?;
                        } else if relation.has_foreign_key {
                            self.remove_linked_values_from_related_relation(relation);
                        } else {
                            object.remove_linked_values_from_related_relation_on_related_object(relation, &object);
                            object.save_with_session_and_path(path).await?;
                        }
                    }
                }
                // value mutation
                let relation_mutation_map = self.inner.relation_mutation_map.lock().await;
                if let Some(manipulation) = relation_mutation_map.get(relation.name()) {
                    if many {
                        self.perform_relation_manipulation_many(relation, manipulation, &(path + relation.name())).await?;
                    } else {
                        self.perform_relation_manipulation_one(relation, manipulation, &(path + relation.name())).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn create_join_object<'a>(&'a self, object: &'a Object, relation: &'static Relation, opposite_relation: &'static Relation, path: &'a KeyPath) -> crate::path::Result<()> {
        let join_model = self.namespace().model_at_path(&relation.through_path().unwrap()).unwrap();
        let action = JOIN_CREATE | CREATE | SINGLE;
        let join_object = self.transaction_ctx().new_object(join_model, action, self.request_ctx())?;
        join_object.set_teon(&teon!({})).await?; // initialize
        let local = relation.local().unwrap();
        let foreign = opposite_relation.local().unwrap();
        let join_local_relation = join_model.relation(local).unwrap();
        self.assign_linked_values_to_related_object(&join_object, join_local_relation);
        let join_foreign_relation = join_model.relation(foreign).unwrap();
        object.assign_linked_values_to_related_object(&join_object, join_foreign_relation);
        match join_object.save_with_session_and_path(path).await {
            Ok(_) => Ok(()),
            Err(_) => Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Can't create join record.")),
        }
    }

    async fn delete_join_object<'a>(&'a self, object: &'a Object, relation: &'static Relation, opposite_relation: &'static Relation, path: &'a KeyPath) -> crate::path::Result<()> {
        let join_model = self.namespace().model_at_path(&relation.through_path().unwrap()).unwrap();
        let action = JOIN_DELETE | DELETE | SINGLE;
        let local = relation.local().unwrap();
        let foreign = opposite_relation.local().unwrap();
        let join_local_relation = join_model.relation(local).unwrap();
        let join_foreign_relation = join_model.relation(foreign).unwrap();
        let mut finder = IndexMap::new();
        for (l, f) in join_local_relation.iter() {
            finder.insert(l.to_owned(), self.get_value(f).unwrap());
        }
        for (l, f) in join_foreign_relation.iter() {
            finder.insert(l.to_owned(), object.get_value(f).unwrap());
        }
        let r#where = Value::Dictionary(finder);
        let object = match self.transaction_ctx().find_unique_internal(join_model, &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Join object is not found.")),
        }.into_not_found_error(path.clone())?;
        match object.delete_from_database(path).await {
            Ok(_) => Ok(()),
            Err(_) => Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Can't delete join record.")),
        }
    }

    fn assign_linked_values_to_related_object(&self, object: &Object, opposite_relation: &'static Relation) {
        for (field, reference) in opposite_relation.iter() {
            object.set_value_to_value_map(field, self.get_value_map_value(reference));
        }
    }

    fn remove_linked_values_from_related_relation(&self, relation: &'static Relation) {
        for (field, _) in relation.iter() {
            self.set_value_to_value_map(field, Value::Null)
        }
    }

    fn remove_linked_values_from_related_relation_on_related_object(&self, relation: &'static Relation, object: &Object) {
        for (_, reference) in relation.iter() {
            object.set_value_to_value_map(reference, Value::Null)
        }
    }

    async fn link_and_save_relation_object(&self, relation: &'static Relation, object: &Object, path: &KeyPath) -> crate::path::Result<()> {
        let mut linked = false;
        let (_, opposite_relation) = self.namespace().opposite_relation(relation);
        if let Some(opposite_relation) = opposite_relation {
            if opposite_relation.has_foreign_key {
                self.assign_linked_values_to_related_object(object, opposite_relation);
                linked = true;
            }
        }
        object.save_with_session_and_path(path).await?;
        if !linked {
            if relation.has_foreign_key {
                object.assign_linked_values_to_related_object(self, relation);
            } else if relation.has_join_table() {
                self.create_join_object(object, relation, opposite_relation.unwrap(), path).await?;
            }
        }
        Ok(())
    }

    async fn nested_create_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let action = NESTED | CREATE | SINGLE;
        let object = self.transaction_ctx().new_object(self.namespace().model_at_path(&relation.model_path()).unwrap(), action, self.request_ctx())?;
        object.set_teon_with_path(value.get("create").unwrap(), path).await?;
        if let Some(opposite) = self.namespace().opposite_relation(relation).1 {
            object.ignore_relation(opposite.name());
        }
        self.link_and_save_relation_object(relation, &object, path).await
    }

    async fn nested_set_many_relation_object_object(&self, relation: &'static Relation, objects: &Vec<Object>, path: &KeyPath) -> Result<()> {
        // disconnect previous
        let records = self.fetch_relation_objects(relation.name(), None).await?;
        for record in records.iter() {
            self.nested_disconnect_relation_object_object(relation, record, path).await?;
        }
        // connect new
        for object in objects {
            self.link_and_save_relation_object(relation, &object, path).await?;
        }
        Ok(())
    }

    async fn nested_set_many_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        // disconnect previous
        let records = self.fetch_relation_objects(relation.name(), None).await?;
        for record in records.iter() {
            self.nested_disconnect_relation_object_object(relation, record, path).await?;
        }
        // connect new
        let value_vec = value.as_array().unwrap();
        for value in value_vec {
            self.nested_connect_relation_object(relation, value, path).await?;
        }
        Ok(())
    }

    async fn nested_set_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        if !(relation.has_foreign_key && relation.is_required()) {
            // disconnect old
            let disconnect_value = self.intrinsic_where_unique_for_relation(relation);
            let _ = self.nested_disconnect_relation_object_no_check(relation, &disconnect_value, path).await;
        }
        if !value.is_null() {
            // connect new
            let action = NESTED | SET | SINGLE;
            let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": value }), true, action, self.request_ctx(), path.clone()).await {
                Ok(object) => object.into_not_found_error(path.clone())?,
                Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Object is not found.")),
            };
            self.link_and_save_relation_object(relation, &object, path).await?;
        }
        Ok(())
    }

    async fn nested_connect_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let action = NESTED | CONNECT | SINGLE;
        let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": value }), true, action, self.request_ctx(), path.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Object is not found.")),
        }.into_not_found_error(path.clone())?;
        self.link_and_save_relation_object(relation, &object, path).await
    }

    async fn nested_connect_or_create_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let r#where = value.get("where").unwrap();
        let create = value.get("create").unwrap();
        let action = CONNECT_OR_CREATE | CONNECT | NESTED | SINGLE;
        let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
            Ok(object) => object.into_not_found_error(path.clone())?,
            Err(_) => {
                self.transaction_ctx().new_object_with_teon_and_path(self.namespace().model_at_path(&relation.model_path()).unwrap(), create, &(path + "create"), action, self.request_ctx()).await?
            },
        };
        self.link_and_save_relation_object(relation, &object, path).await
    }

    fn intrinsic_where_unique_for_relation(&self, relation: &'static Relation) -> Value {
        Value::Dictionary(relation.iter().map(|(l, f)| (f.to_owned(), self.get_value(l).unwrap())).collect())
    }

    fn intrinsic_where_unique_for_opposite_relation(&self, relation: &'static Relation) -> Value {
        Value::Dictionary(relation.iter().map(|(l, f)| (l.to_owned(), self.get_value(f).unwrap())).collect())
    }

    async fn nested_disconnect_relation_object_object(&self, relation: &'static Relation, object: &Object, path: &KeyPath) -> crate::path::Result<()> {
        if !relation.is_vec && relation.is_required() {
            return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Cannot disconnect required relation."));
        }
        if relation.has_foreign_key {
            self.remove_linked_values_from_related_relation(relation);
        } else if relation.has_join_table() {
            self.delete_join_object(object, relation, self.namespace().opposite_relation(relation).1.unwrap(), path).await?;
        } else {
            object.remove_linked_values_from_related_relation_on_related_object(relation, &object);
            object.save_with_session_and_path(path).await?;
        }
        Ok(())
    }

    async fn nested_disconnect_relation_object_no_check(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        if relation.has_foreign_key {
            self.remove_linked_values_from_related_relation(relation);
        } else {
            let r#where = value;
            let action = NESTED | DISCONNECT | SINGLE;
            let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
                Ok(object) => object,
                Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "object is not found")),
            }.into_not_found_error(path.clone())?;
            object.remove_linked_values_from_related_relation_on_related_object(relation, &object);
            object.save_with_session_and_path(path).await?;
        }
        Ok(())
    }

    async fn nested_disconnect_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        if !relation.is_vec && relation.is_required() {
            return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Cannot disconnect required relation."));
        }
        self.nested_disconnect_relation_object_no_check(relation, value, path).await?;
        Ok(())
    }

    async fn nested_upsert_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let mut r#where = self.intrinsic_where_unique_for_relation(relation);
        r#where.as_dictionary_mut().unwrap().extend(value.get("where").unwrap().as_dictionary().cloned().unwrap());
        let create = value.get("create").unwrap();
        let update = value.get("update").unwrap();
        let action = NESTED | UPSERT | UPDATE | SINGLE;
        match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await.into_not_found_error(path.clone()) {
            Ok(object) => {
                let path = path + "update";
                object.set_teon_with_path(update, &path).await?;
                object.save_with_session_and_path(&path).await?;
            },
            Err(_) => {
                let action = NESTED | UPSERT | CREATE | SINGLE;
                let object = self.transaction_ctx().new_object_with_teon_and_path(self.namespace().model_at_path(&relation.model_path()).unwrap(), create, &(path + "create"), action, self.request_ctx()).await?;
                self.link_and_save_relation_object(relation, &object, path).await?;
            },
        };
        Ok(())
    }

    async fn nested_many_disconnect_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        if relation.has_join_table() {
            let action = JOIN_DELETE | DELETE | SINGLE;
            let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": value }), true, action, self.request_ctx(), path.clone()).await {
                Ok(object) => object.into_not_found_error(path.clone())?,
                Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Object is not found.")),
            };
            self.delete_join_object(&object, relation, self.namespace().opposite_relation(relation).1.unwrap(), path).await?;
        } else {
            let mut r#where = self.intrinsic_where_unique_for_relation(relation);
            r#where.as_dictionary_mut().unwrap().extend(value.as_dictionary().cloned().unwrap().into_iter());
            let action = DISCONNECT | NESTED | SINGLE;
            let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
                Ok(object) => object.into_not_found_error(path.clone())?,
                Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Object is not found.")),
            };
            object.remove_linked_values_from_related_relation_on_related_object(relation, &object);
            object.save_with_session_and_path(path).await?;
        }
        Ok(())
    }

    async fn find_relation_objects_by_value(&self, relation: &'static Relation, value: &Value, path: &KeyPath, action: Action) -> crate::path::Result<Vec<Object>> {
        if relation.has_join_table() {
            let mut finder = IndexMap::new();
            let join_relation = self.namespace().through_relation(relation).1;
            for (l, f) in join_relation.iter() {
                finder.insert(l.to_owned(), self.get_value(f).unwrap());
            }
            finder.insert(self.namespace().through_opposite_relation(relation).1.name().to_owned(), teon!({
                "is": value
            }));
            if let Ok(join_objects) = self.transaction_ctx().find_many_internal(self.namespace().model_at_path(&relation.through_path().unwrap()).unwrap(), &teon!({
                "where": Value::Dictionary(finder),
                "include": {
                    self.namespace().through_opposite_relation(relation).1.name(): true
                }
            }), true, action, self.request_ctx(), path.clone()).await {
                let mut results = vec![];
                for join_object in join_objects {
                    let object = join_object.get_query_relation_object(self.namespace().through_opposite_relation(relation).1.name(), path)?.unwrap();
                    results.push(object);
                }
                Ok(results)
            } else {
                return Err(error_ext::unexpected_input_value_with_reason((path + "where"), "Object is not found."));
            }
        } else {
            let mut r#where = self.intrinsic_where_unique_for_relation(relation);
            r#where.as_dictionary_mut().unwrap().extend(value.as_dictionary().cloned().unwrap());
            let action = NESTED | UPDATE | MANY;
            let objects = self.transaction_ctx().find_many_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await.unwrap();
            Ok(objects)
        }
    }

    async fn find_relation_object_by_value(&self, relation: &'static Relation, value: &Value, path: &KeyPath, action: Action) -> crate::path::Result<Object> {
        if relation.has_join_table() {
            let mut finder = IndexMap::new();
            let join_relation = self.namespace().through_relation(relation).1;
            for (l, f) in join_relation.iter() {
                finder.insert(l.to_owned(), self.get_value(f).unwrap());
            }
            finder.insert(self.namespace().through_opposite_relation(relation).1.name().to_owned(), teon!({
                "is": value
            }));
            if let Ok(join_object) = self.transaction_ctx().find_first_internal(self.namespace().model_at_path(&relation.through_path().unwrap()).unwrap(), &teon!({
                "where": Value::Dictionary(finder),
                "include": {
                    self.namespace().through_opposite_relation(relation).1.name(): true
                }
            }), true, action, self.request_ctx(), path.clone()).await.into_not_found_error(path.clone()) {
                let object = join_object.get_query_relation_object(self.namespace().through_opposite_relation(relation).1.name(), path)?.unwrap();
                Ok(object)
            } else {
                return Err(error_ext::unexpected_input_value_with_reason((path + "where"), "Object is not found."));
            }
        } else {
            let mut r#where = self.intrinsic_where_unique_for_relation(relation);
            r#where.as_dictionary_mut().unwrap().extend(value.as_dictionary().cloned().unwrap());
            let action = NESTED | UPDATE | SINGLE;
            let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
                Ok(object) => object,
                Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path + "where", "Object is not found.")),
            }.into_not_found_error(path.clone())?;
            Ok(object)
        }
    }

    async fn nested_many_update_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let object = self.find_relation_object_by_value(relation, value.get("where").unwrap(), path, NESTED | UPDATE | SINGLE).await?;
        object.set_teon_with_path(value.get("update").unwrap(), &(path + "update")).await?;
        object.save_with_session_and_path(path).await?;
        Ok(())
    }

    async fn nested_many_update_many_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let objects = self.find_relation_objects_by_value(relation, value.get("where").unwrap(), path, NESTED | UPDATE | MANY).await?;
        let update = value.get("update").unwrap();
        for object in objects {
            object.set_teon_with_path(update, path).await?;
            object.save_with_session_and_path(path).await?;
        }
        Ok(())
    }

    async fn nested_update_relation_object<'a>(&'a self, relation: &'static Relation, value: &'a Value, path: &'a KeyPath) -> crate::path::Result<()> {
        let r#where = value.get("where").unwrap();
        let action = NESTED | UPDATE | SINGLE;
        let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
            Ok(object) => object.into_not_found_error(path.clone())?,
            Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "update: object not found")),
        };
        object.set_teon_with_path(value.get("update").unwrap(), path).await?;
        object.save_with_session_and_path(path).await?;
        Ok(())
    }

    async fn nested_delete_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        if !relation.is_vec && relation.is_required() {
            return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "Cannot delete required relation."));
        }
        let r#where = value.get("where").unwrap();
        let action = NESTED | DELETE | SINGLE;
        let object = match self.transaction_ctx().find_unique_internal(self.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({ "where": r#where }), true, action, self.request_ctx(), path.clone()).await {
            Ok(object) => object.into_not_found_error(path.clone())?,
            Err(_) => return Err(error_ext::unexpected_input_value_with_reason(path.clone(), "delete: object not found")),
        };
        object.delete_from_database(path).await?;
        if relation.has_join_table() {
            let opposite_relation = self.namespace().opposite_relation(relation).1.unwrap();
            self.delete_join_object(&object, relation, opposite_relation, path).await?;
        }
        if relation.has_foreign_key {
            self.remove_linked_values_from_related_relation(relation);
        }
        Ok(())
    }

    async fn nested_many_delete_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let object = self.find_relation_object_by_value(relation, value, path, NESTED | DELETE | SINGLE).await?;
        object.delete_from_database(path).await?;
        if relation.has_join_table() {
            let opposite_relation = self.namespace().opposite_relation(relation).1.unwrap();
            self.delete_join_object(&object, relation, opposite_relation, path).await?;
        }
        Ok(())
    }

    async fn nested_many_delete_many_relation_object(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        let objects = self.find_relation_objects_by_value(relation, value, path, NESTED | DELETE | MANY).await?;
        for object in objects {
            object.delete_from_database(path).await?;
            if relation.has_join_table() {
                let opposite_relation = self.namespace().opposite_relation(relation).1.unwrap();
                self.delete_join_object(&object, relation, opposite_relation, path).await?;
            }
        }
        Ok(())
    }

    async fn disconnect_object_which_connects_to<'a>(&'a self, relation: &'static Relation, value: &'a Value, path: &KeyPath) -> crate::path::Result<()> {
        if let Ok(that) = self.transaction_ctx().find_unique::<Object>(self.model(), &teon!({
            "where": {
                relation.name(): {
                    "is": value
                }
            }
        }), self.request_ctx(), path.clone()).await.into_not_found_error(path.clone()) {
            if relation.is_required() {
                return Err(error_ext::cannot_disconnect_previous_relation(path.clone()));
            } else {
                for (l, _f) in relation.iter() {
                    that.set_value(l, Value::Null).unwrap();
                }
                that.save().await.unwrap();
            }
        }
        Ok(())
    }

    async fn perform_relation_manipulation_one_inner(&self, relation: &'static Relation, action: Action, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        if !relation.is_vec && !relation.has_foreign_key && !self.is_new() {
            match action {
                NESTED_CREATE_ACTION | NESTED_CONNECT_ACTION | NESTED_CONNECT_OR_CREATE_ACTION => {
                    let disconnect_value = self.intrinsic_where_unique_for_relation(relation);
                    let _ = self.nested_disconnect_relation_object_no_check(relation, &disconnect_value, path).await;
                },
                _ => ()
            }
        }
        if !relation.is_vec && relation.has_foreign_key {
            if let Some(opposite_relation) = self.namespace().opposite_relation(relation).1 {
                if !opposite_relation.is_vec {
                    match action {
                        NESTED_CONNECT_ACTION | NESTED_SET_ACTION => {
                            if !value.is_null() {
                                self.disconnect_object_which_connects_to(relation, value, path).await?;
                            }
                        }
                        NESTED_CONNECT_OR_CREATE_ACTION => {
                            self.disconnect_object_which_connects_to(relation, value.get("where").unwrap(), path).await?;
                        }
                        _ => ()
                    }
                }
            }
        }
        match action {
            NESTED_CREATE_ACTION => self.nested_create_relation_object(relation, value, &path).await,
            NESTED_CONNECT_ACTION => self.nested_connect_relation_object(relation, value, &path).await,
            NESTED_SET_ACTION => self.nested_set_relation_object(relation, value, &path).await,
            NESTED_CONNECT_OR_CREATE_ACTION => self.nested_connect_or_create_relation_object(relation, value, &path).await,
            NESTED_DISCONNECT_ACTION => self.nested_disconnect_relation_object(relation, value, &path).await,
            NESTED_UPDATE_ACTION => self.nested_update_relation_object(relation, value, &path).await,
            NESTED_DELETE_ACTION => self.nested_delete_relation_object(relation, value, &path).await,
            NESTED_UPSERT_ACTION => self.nested_upsert_relation_object(relation, value, &path).await,
            _ => unreachable!(),
        }
    }

    fn normalize_relation_one_value<'a>(&'a self, relation: &'static Relation, action: Action, value: &'a Value) -> Cow<Value> {
        match action {
            NESTED_CREATE_ACTION => Owned(Value::Dictionary(indexmap! {"create".to_owned() => value.clone()})),
            NESTED_UPDATE_ACTION => Owned(Value::Dictionary(indexmap! {"update".to_owned() => value.clone(), "where".to_owned() => self.intrinsic_where_unique_for_relation(relation)})),
            NESTED_DELETE_ACTION => Owned(Value::Dictionary(indexmap! {"where".to_owned() => self.intrinsic_where_unique_for_relation(relation)})),
            NESTED_DISCONNECT_ACTION => Owned(self.intrinsic_where_unique_for_relation(relation)),
            NESTED_UPSERT_ACTION => {
                let mut value = value.clone();
                value.as_dictionary_mut().unwrap().insert("where".to_owned(), self.intrinsic_where_unique_for_relation(relation));
                Owned(value)
            }
            _ => Borrowed(value)
        }
    }

    async fn perform_relation_manipulation_one(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> Result<()> {
        for (key, value) in value.as_dictionary().unwrap() {
            let key = key.as_str();
            let path = path + key;
            let action = Action::nested_from_name(key).unwrap();
            let other_model = self.namespace().opposite_relation(relation).0;
            let normalized_value = self.normalize_relation_one_value(relation, action, value);
            // todo: action transform
            //let ctx = PipelineCtx::initial_state_with_value(normalized_value.as_ref().clone(), self.transaction_ctx().transaction_for_model(self.model()).unwrap(), self.initiator().as_req()).with_path(path.clone()).with_action(action);
            //let (transformed_value, new_action) = other_model.transformed_action(ctx).await?;
            self.perform_relation_manipulation_one_inner(relation, action, &normalized_value, &path).await?;
        }
        Ok(())
    }

    fn normalize_relation_many_value<'a>(&'a self, action: Action, value: &'a Value) -> Cow<Value> {
        match action {
            NESTED_CREATE_ACTION => Owned(Value::Dictionary(indexmap! {"create".to_owned() => value.clone()})),
            _ => Borrowed(value)
        }
    }

    async fn perform_relation_manipulation_many_inner(&self, relation: &'static Relation, action: Action, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        match action {
            NESTED_CREATE_ACTION => self.nested_create_relation_object(relation, value, &path).await,
            NESTED_CONNECT_ACTION => self.nested_connect_relation_object(relation, value, &path).await,
            NESTED_SET_ACTION => self.nested_set_many_relation_object(relation, value, &path).await,
            NESTED_CONNECT_OR_CREATE_ACTION => self.nested_connect_or_create_relation_object(relation, value, &path).await,
            NESTED_DISCONNECT_ACTION => self.nested_many_disconnect_relation_object(relation, value, &path).await,
            NESTED_UPSERT_ACTION => self.nested_upsert_relation_object(relation, value, &path).await,
            NESTED_UPDATE_ACTION => self.nested_many_update_relation_object(relation, value, &path).await,
            NESTED_UPDATE_MANY_ACTION => self.nested_many_update_many_relation_object(relation, value, &path).await,
            NESTED_DELETE_ACTION => self.nested_many_delete_relation_object(relation, value, &path).await,
            NESTED_DELETE_MANY_ACTION => self.nested_many_delete_many_relation_object(relation, value, &path).await,
            _ => unreachable!(),
        }
    }

    async fn perform_relation_manipulation_many(&self, relation: &'static Relation, value: &Value, path: &KeyPath) -> crate::path::Result<()> {
        for (key, value) in value.as_dictionary().unwrap() {
            let key = key.as_str();
            let path = path + key;
            let action = Action::nested_from_name(key).unwrap();
            let other_model = self.namespace().opposite_relation(relation).0;
            if value.is_array() && action != NESTED_SET_ACTION {
                for (index, value) in value.as_array().unwrap().iter().enumerate() {
                    let normalized_value = self.normalize_relation_many_value(action, value);
                    // todo: transform action
                    //let ctx = PipelineCtx::initial_state_with_value(normalized_value.as_ref().clone(), self.transaction_ctx().transaction_for_model(self.model()).unwrap(), self.initiator().as_req()).with_path(&(path.clone() + index)).with_action(action);
                    //let (transformed_value, new_action) = other_model.transformed_action(ctx).await?;
                    self.perform_relation_manipulation_many_inner(relation, action, &normalized_value, &path).await?;
                }
            }  else {
                let normalized_value = self.normalize_relation_many_value(action, value);
                // todo: transform action
                //let ctx = PipelineCtx::initial_state_with_value(normalized_value.as_ref().clone(), self.transaction_ctx().transaction_for_model(self.model()).unwrap(), self.initiator().as_req()).with_path(path.clone()).with_action(action);
                //let (transformed_value, new_action) = other_model.transformed_action(ctx).await?;
                self.perform_relation_manipulation_many_inner(relation, action, &normalized_value, &path).await?;
            }
        }
        Ok(())
    }

    pub async fn refreshed(&self, include: Option<&Value>, select: Option<&Value>) -> Result<Object> {
        let mut finder = teon!({
            "where": self.identifier(),
        });
        if let Some(include) = include {
            finder.as_dictionary_mut().unwrap().insert("include".to_string(), include.clone());
        }
        if let Some(select) = select {
            finder.as_dictionary_mut().unwrap().insert("select".to_string(), select.clone());
        }
        let target = self.transaction_ctx().find_unique_internal(self.model(), &finder, false, self.action(), self.request_ctx(), path![]).await.into_not_found_error(path![]);
        match target {
            Ok(obj) => {
                if self.model().cache.has_virtual_fields {
                    self.copy_virtual_fields(&obj);
                }
                Ok(obj)
            }
            Err(err) => Err(err.into())
        }
    }

    fn copy_virtual_fields(&self, other: &Object) {
        for field in self.model().fields() {
            if field.r#virtual {
                let result = self.get_value(field.name()).unwrap();
                other.set(field.name(), result).unwrap();
            }
        }
    }

    pub async fn force_set_relation_objects(&self, key: &str, objects: Vec<Object>) -> () {
        self.inner.object_set_many_map.lock().await.insert(key.to_owned(), objects);
    }

    pub async fn force_add_relation_objects(&self, key: &str, objects: Vec<Object>) -> () {
        self.inner.object_connect_map.lock().await.insert(key.to_owned(), objects);
    }

    pub async fn force_remove_relation_objects(&self, key: &str, objects: Vec<Object>) -> () {
        self.inner.object_disconnect_map.lock().await.insert(key.to_owned(), objects);
    }

    pub async fn force_get_relation_objects(&self, key: &str, find_many_args: impl AsRef<Value>) -> crate::path::Result<Vec<Object>> {
        self.fetch_relation_objects(key, Some(find_many_args.as_ref())).await
    }

    pub async fn force_set_relation_object(&self, key: &str, object: Option<Object>) -> () {
        self.inner.object_set_map.lock().await.insert(key.to_owned(), object);
    }

    pub async fn force_get_relation_object(&self, key: &str) -> Result<Option<Object>> {
        if self.has_mutation_relation_fetched(key) {
            self.get_mutation_relation_object(key)
        } else {
            match self.fetch_relation_object(key, None).await {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }
    }

    pub async fn fetch_relation_object(&self, key: impl AsRef<str>, find_unique_arg: Option<&Value>) -> crate::path::Result<Option<Object>> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        let mut finder = self.intrinsic_where_unique_for_relation(relation);
        if let Some(find_unique_arg) = find_unique_arg {
            if let Some(include) = find_unique_arg.get("include") {
                finder.as_dictionary_mut().unwrap().insert("include".to_owned(), include.clone());
            }
            if let Some(select) = find_unique_arg.get("select") {
                finder.as_dictionary_mut().unwrap().insert("select".to_owned(), select.clone());
            }
        }
        let relation_model_name = self.namespace().model_at_path(&relation.model_path()).unwrap();
        let action = NESTED | FIND | CODE_NAME | SINGLE;
        match self.transaction_ctx().find_unique_internal(relation_model_name, &finder, false, action, self.request_ctx(), path![]).await {
            Ok(result) => {
                self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), vec![result.into_not_found_error(path![])?]);
                let obj = self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().get(0).unwrap().clone();
                Ok(Some(obj.clone()))
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn fetch_relation_objects(&self, key: impl AsRef<str>, find_many_arg: Option<&Value>) -> crate::path::Result<Vec<Object>> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        let empty = teon!({});
        let include_inside = if find_many_arg.is_some() {
            find_many_arg.unwrap()
        } else {
            &empty
        };
        let action = CODE_POSITION | CODE_NAME | FIND | MANY;
        if relation.has_join_table() {
            let identifier = self.identifier();
            let new_self = self.transaction_ctx().find_unique_internal(model, &teon!({
                "where": identifier,
                "include": {
                    key.as_ref(): include_inside
                }
            }), false, action, self.request_ctx(), path![]).await.into_not_found_error(path![])?;
            let vec = new_self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().clone();
            Ok(vec)
        } else {
            let mut finder = teon!({});
            if let Some(find_many_arg) = find_many_arg {
                for (k, v) in find_many_arg.as_dictionary().unwrap().iter() {
                    finder.as_dictionary_mut().unwrap().insert(k.clone(), v.clone());
                }
            }
            if finder.as_dictionary().unwrap().get("where").is_none() {
                finder.as_dictionary_mut().unwrap().insert("where".to_string(), teon!({}));
            }
            for (index, local_field_name) in relation.fields.iter().enumerate() {
                let foreign_field_name = relation.references.get(index).unwrap();
                let value = self.get_value(local_field_name).unwrap();
                if value == Value::Null {
                    return Ok(vec![]);
                }
                let json_value = value;
                finder.as_dictionary_mut().unwrap().get_mut("where").unwrap().as_dictionary_mut().unwrap().insert(foreign_field_name.to_owned(), json_value);
            }
            let relation_model = self.namespace().model_at_path(&relation.model_path()).unwrap();
            let results = self.transaction_ctx().find_many_internal(relation_model, &finder, false, action, self.request_ctx(), path![]).await?;
            Ok(results)
        }
    }

    pub fn keys_for_save(&self) -> Vec<&str> {
        if self.is_new() {
            self.model().cache.save_keys.iter().map(|k| k.as_str()).collect()
        } else {
            self.model().cache.save_keys.iter().filter(|k| {
                self.inner.modified_fields.lock().unwrap().contains(&k.to_string()) ||
                    self.inner.atomic_updater_map.lock().unwrap().contains_key(&k.to_string())
            }).map(|k| k.as_str()).collect()
        }
    }

    pub fn action(&self) -> Action {
        self.inner.action
    }

    pub fn ignore_relation(&self, name: &str) {
        *self.inner.ignore_relation.lock().unwrap() = Some(name.to_owned()); 
    }
}

#[derive(Debug)]
pub struct ObjectInner {
    request_ctx: Option<request::Ctx>,
    transaction_ctx: transaction::Ctx,
    model: &'static Model,
    action: Action,
    pub is_initialized: AtomicBool,
    pub is_new: AtomicBool,
    is_modified: AtomicBool,
    is_partial: AtomicBool,
    is_deleted: AtomicBool,
    inside_before_save_callback: AtomicBool,
    inside_after_save_callback: AtomicBool,
    selected_fields: Arc<Mutex<Vec<String>>>,
    modified_fields: Arc<Mutex<BTreeSet<String>>>,
    value_map: Arc<Mutex<BTreeMap<String, Value>>>,
    previous_value_map: Arc<Mutex<BTreeMap<String, Value>>>,
    pub atomic_updater_map: Arc<Mutex<BTreeMap<String, Value>>>,
    pub relation_mutation_map: Arc<TokioMutex<BTreeMap<String, Value>>>,
    pub relation_query_map: Arc<Mutex<BTreeMap<String, Vec<Object>>>>,
    pub cached_property_map: Arc<Mutex<BTreeMap<String, Value>>>,
    pub object_set_map: Arc<TokioMutex<BTreeMap<String, Option<Object>>>>,
    pub object_set_many_map: Arc<TokioMutex<BTreeMap<String, Vec<Object>>>>,
    pub object_connect_map: Arc<TokioMutex<BTreeMap<String, Vec<Object>>>>,
    pub object_disconnect_map: Arc<TokioMutex<BTreeMap<String, Vec<Object>>>>,
    ignore_relation: Arc<Mutex<Option<String>>>,
}

impl Serialize for Object {

    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}

fn check_user_json_keys<'a>(map: &IndexMap<String, Value>, allowed: &HashSet<&str>, model: &'static Model) -> Result<()> {
    if let Some(unallowed) = map.keys().find(|k| !allowed.contains(k.as_str())) {
        return Err(Error::new(format!("key '{}' is not allowed for {}", unallowed, model.name())));
    }
    Ok(())
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct(self.model().name());
        for field in self.model().fields() {
            let map = self.inner.value_map.lock().unwrap();
            let value = map.get(field.name()).unwrap_or(&Value::Null);
            result.field(field.name(), value);
        }
        result.finish()
    }
}

impl Display for Object {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} {{ {} }}", self.model().name(), self.model().fields().iter().map(|field| {
            let map = self.inner.value_map.lock().unwrap();
            let value = map.get(field.name()).unwrap_or(&Value::Null);
            format!("{}: {}", field.name(), value)
        }).join(", ")).as_str())
    }
}

impl PartialEq for Object {

    fn eq(&self, other: &Self) -> bool {
        self.model() == other.model() && self.identifier() == other.identifier()
    }
}

unsafe impl Send for Object { }
unsafe impl Sync for Object { }

pub trait ErrorIfNotFound {
    fn into_not_found_error(self, path: KeyPath) -> crate::path::Result<Object>;
}

impl ErrorIfNotFound for Option<Object> {
    fn into_not_found_error(self, path: KeyPath) -> crate::path::Result<Object> {
        match self {
            Some(object) => Ok(object),
            None => Err(error_ext::not_found(path)),
        }
    }
}

impl ErrorIfNotFound for crate::path::Result<Option<Object>> {

    fn into_not_found_error(self, path: KeyPath) -> crate::path::Result<Object> {
        match self {
            Err(err) => Err(err),
            Ok(option) => match option {
                Some(object) => Ok(object),
                None => Err(error_ext::not_found(path)),
            }
        }
    }
}

impl ErrorIfNotFound for Result<Option<Object>> {
    fn into_not_found_error(self, path: KeyPath) -> crate::path::Result<Object> {
        match self {
            Err(err) => Err(crate::path::Error::value_error(path, format!("{}", err))),
            Ok(option) => match option {
                Some(object) => Ok(object),
                None => Err(error_ext::not_found(path)),
            }
        }
    }
}

unsafe impl Send for ObjectInner {}
unsafe impl Sync for ObjectInner {}


