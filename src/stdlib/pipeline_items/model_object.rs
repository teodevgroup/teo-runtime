use std::collections::HashMap;
use indexmap::IndexMap;
use num_integer::Integer;
use teo_teon::types::range::Range;
use teo_teon::Value;
use crate::arguments::Arguments;
use teo_result::Error;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use teo_result::{Result, ResultExt};
use rand::{thread_rng, Rng};
use teo_teon::types::enum_variant::EnumVariant;
use crate::model;

pub(in crate::stdlib) fn load_pipeline_model_object_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("self", |args: Arguments, ctx: Ctx| async move {
        Ok(Object::from(ctx.object()))
    });

    namespace.define_pipeline_item("get", |args: Arguments, ctx: Ctx| async move {
        let model_object: Result<&model::Object> = ctx.value().try_into_err_prefix("get");
        let dictionary: Result<&IndexMap<String, Value>> = ctx.value().try_into_err_prefix("get");
        if let Ok(model_object) = model_object {
            let key: &EnumVariant = args.get("key").err_prefix("get(key)")?;
            let key_value = key.value.as_str();
            // get value and return here
            let value: Value = model_object.get_value(key_value)?;
            Ok(Object::from(value))
        } else if let Ok(dictionary) = dictionary {
            let key: &str = args.get("key").err_prefix("get(key)")?;
            Ok(Object::from(dictionary.get(key).cloned().unwrap_or(Value::Null)))
        } else {
            Err(Error::new("get: input is not model object or dictionary"))
        }
    });

    namespace.define_pipeline_item("set", |args: Arguments, ctx: Ctx| async move {
        let model_object: Result<&model::Object> = ctx.value().try_into_err_prefix("set");
        let dictionary: Result<&IndexMap<String, Value>> = ctx.value().try_into_err_prefix("set");
        let value: Value = args.get("value")?;
        if let Ok(model_object) = model_object {
            let key: &EnumVariant = args.get("key").err_prefix("set(key)")?;
            let key_value = key.value.as_str();
            // get value and return here
            model_object.set_value(key_value, value)?;
            Ok(ctx.value().clone())
        } else if let Ok(dictionary) = dictionary {
            let key: &str = args.get("key").err_prefix("set(key)")?;
            let mut new_dictionary = dictionary.clone();
            new_dictionary.insert(key.to_owned(), value);
            Ok(Object::from(Value::Dictionary(new_dictionary)))
        } else {
            Err(Error::new("set: input is not model object or dictionary"))
        }
    });

    namespace.define_pipeline_item("assign", |args: Arguments, ctx: Ctx| async move {
        let model_object = ctx.object();
        let value: Value = args.get("value")?;
        let key: &EnumVariant = args.get("key").err_prefix("assign(key)")?;
        let key_value = key.value.as_str();
        // get value and return here
        model_object.set_value(key_value, value)?;
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("previous", |args: Arguments, ctx: Ctx| async move {
        let model_object = ctx.object();
        let key: &EnumVariant = args.get("key").err_prefix("previous(key)")?;
        let key_value = key.value.as_str();
        // get value and return here
        let result = model_object.get_previous_value(key_value)?;
        Ok(Object::from(result))
    });
}