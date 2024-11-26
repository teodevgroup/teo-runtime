use indexmap::IndexMap;
use crate::value::Value;
use crate::arguments::Arguments;
use teo_result::Error;
use crate::pipeline::Ctx;
use teo_result::{Result, ResultExt};
use crate::{model, namespace};
use crate::pipeline::item::item_impl::ItemImpl;

pub(in crate::stdlib) fn load_pipeline_model_object_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("self", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            Ok(Value::from(ctx.object()))
        }))
    });

    namespace.define_pipeline_item("get", |args: Arguments| {
        let key: String = args.get("key").error_message_prefixed("get(key)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let key = key.clone();
            async move {
                let model_object: Result<&model::Object> = ctx.value().try_ref_into_err_prefix("get");
                let dictionary: Result<&IndexMap<String, Value>> = ctx.value().try_ref_into_err_prefix("get");
                if let Ok(model_object) = model_object {
                    let value: Value = model_object.get_value(key)?;
                    Ok(Value::from(value))
                } else if let Ok(dictionary) = dictionary {
                    Ok(Value::from(dictionary.get(&key).cloned().unwrap_or(Value::Null)))
                } else {
                    Err(Error::new("get: input is not model object or dictionary"))
                }
            }
        }))
    });

    namespace.define_pipeline_item("set", |args: Arguments| {
        let key: String = args.get("key").error_message_prefixed("set(key)")?;
        let value: Value = args.get("value").error_message_prefixed("set(value)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let key = key.clone();
            let value = value.clone();
            async move {
                let model_object: Result<&model::Object> = ctx.value().try_ref_into_err_prefix("set");
                let dictionary: Result<&IndexMap<String, Value>> = ctx.value().try_ref_into_err_prefix("set");
                if let Ok(model_object) = model_object {
                    model_object.set_value(key, value)?;
                    Ok(ctx.value().clone())
                } else if let Ok(dictionary) = dictionary {
                    let mut new_dictionary = dictionary.clone();
                    new_dictionary.insert(key.clone(), value);
                    Ok(Value::from(Value::Dictionary(new_dictionary)))
                } else {
                    Err(Error::new("set: input is not model object or dictionary"))
                }
            }
        }))
    });

    namespace.define_pipeline_item("assign", |args: Arguments| {
        let key: String = args.get("key").error_message_prefixed("assign(key)")?;
        let value: Value = args.get("value").error_message_prefixed("assign(value)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let key = key.clone();
            let value = value.clone();
            async move {
                let model_object = ctx.object();
                model_object.set_value(&key, value)?;
                Ok(ctx.value().clone())
            }
        }))
    });

    namespace.define_pipeline_item("previous", |args: Arguments| {
        let key: String = args.get("key").error_message_prefixed("previous(key)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let key = key.clone();
            async move {
                let model_object = ctx.object();
                let result = model_object.get_previous_value(key)?;
                Ok(Value::from(result))
            }
        }))
    });
}