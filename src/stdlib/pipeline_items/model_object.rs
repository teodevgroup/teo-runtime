use indexmap::IndexMap;
use crate::value::Value;
use crate::arguments::Arguments;
use teo_result::Error;
use crate::pipeline::Ctx;
use teo_result::{Result, ResultExt};
use crate::{model, namespace};

pub(in crate::stdlib) fn load_pipeline_model_object_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("self", |args: Arguments, ctx: Ctx| async move {
        Ok(Value::from(ctx.object()))
    });

    namespace.define_pipeline_item("get", |args: Arguments, ctx: Ctx| async move {
        let model_object: Result<&model::Object> = ctx.value().try_ref_into_err_prefix("get");
        let dictionary: Result<&IndexMap<String, Value>> = ctx.value().try_ref_into_err_prefix("get");
        if let Ok(model_object) = model_object {
            let key: &str = args.get("key").error_message_prefixed("get(key)")?;
            // get value and return here
            let value: Value = model_object.get_value(key)?;
            Ok(Value::from(value))
        } else if let Ok(dictionary) = dictionary {
            let key: &Value = args.get("key").error_message_prefixed("get(key)")?;
            let key_str = if key.is_string() {
                key.as_str().unwrap()
            } else {
                unreachable!()
            };
            Ok(Value::from(dictionary.get(key_str).cloned().unwrap_or(Value::Null)))
        } else {
            Err(Error::new("get: input is not model object or dictionary"))
        }
    });

    namespace.define_pipeline_item("set", |args: Arguments, ctx: Ctx| async move {
        let model_object: Result<&model::Object> = ctx.value().try_ref_into_err_prefix("set");
        let dictionary: Result<&IndexMap<String, Value>> = ctx.value().try_ref_into_err_prefix("set");
        let value: Value = args.get("value")?;
        if let Ok(model_object) = model_object {
            let key: &str = args.get("key").error_message_prefixed("set(key)")?;
            // get value and return here
            model_object.set_value(key, value)?;
            Ok(ctx.value().clone())
        } else if let Ok(dictionary) = dictionary {
            let key: &str = args.get("key").error_message_prefixed("set(key)")?;
            let mut new_dictionary = dictionary.clone();
            new_dictionary.insert(key.to_owned(), value);
            Ok(Value::from(Value::Dictionary(new_dictionary)))
        } else {
            Err(Error::new("set: input is not model object or dictionary"))
        }
    });

    namespace.define_pipeline_item("assign", |args: Arguments, ctx: Ctx| async move {
        let model_object = ctx.object();
        let value: Value = args.get("value")?;
        let key: &str = args.get("key").error_message_prefixed("assign(key)")?;
        // get value and return here
        model_object.set_value(key, value)?;
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("previous", |args: Arguments, ctx: Ctx| async move {
        let model_object = ctx.object();
        let key: &str = args.get("key").error_message_prefixed("previous(key)")?;
        // get value and return here
        let result = model_object.get_previous_value(key)?;
        Ok(Value::from(result))
    });
}