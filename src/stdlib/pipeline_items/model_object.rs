use std::collections::HashMap;
use indexmap::IndexMap;
use num_integer::Integer;
use teo_teon::types::range::Range;
use teo_teon::Value;
use crate::arguments::Arguments;
use crate::error::Error;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::result::{Result, ResultExt};
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
            // key.value.as_ref().as_str()
            // get value and return here
            //model_object.get()
            Ok(Object::from(ctx.object()))
        } else if let Ok(dictionary) = dictionary {
            let key: &str = args.get("key").err_prefix("get(key)")?;
            Ok(Object::from(dictionary.get(key).cloned().unwrap_or(Value::Null)))
        } else {
            Err(Error::new("get: input is not model object or dictionary"))
        }
    });
}