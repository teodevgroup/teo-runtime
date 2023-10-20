use std::ops::Not;
use teo_teon::Value;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::action::Action;
use crate::pipeline::pipeline::Pipeline;

pub(in crate::stdlib) fn load_model_decorators(namespace: &mut Namespace) {

    namespace.define_model_decorator("map", |arguments, model| {
        let table_name: String = arguments.get("tableName")?;
        let mut namespace_prefix = model.namespace_path().join("_");
        if !namespace_prefix.is_empty() {
            namespace_prefix += "__";
        }
        model.table_name = namespace_prefix + &table_name;
        Ok(())
    });

    namespace.define_model_decorator("id", |arguments, model| {
       Ok(())
    });

    namespace.define_model_decorator("index", |arguments, model| {
        Ok(())
    });

    namespace.define_model_decorator("unique", |arguments, model| {
        Ok(())
    });

    namespace.define_model_decorator("migration", |arguments, model| {
        let table_name: Value = arguments.get("tableName")?;
        let version: Result<String> = arguments.get("version");
        let drop: Result<bool> = arguments.get("drop");
        if table_name.is_string() {
            model.migration.renamed = Some(vec![table_name.as_str().unwrap().to_owned()]);
        } else if table_name.is_array() {
            model.migration.renamed = Some(table_name.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_owned()).collect());
        }
        if let Ok(version) = version {
            model.migration.version = Some(version);
        }
        if let Ok(drop) = drop {
            model.migration.drop = drop;
        } else {
            model.migration.drop = false;
        }
        Ok(())
    });

    namespace.define_model_decorator("beforeSave", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.before_save = pipeline;
        Ok(())
    });

    namespace.define_model_decorator("afterSave", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.after_save = pipeline;
        Ok(())
    });

    namespace.define_model_decorator("beforeDelete", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.before_delete = pipeline;
        Ok(())
    });

    namespace.define_model_decorator("afterDelete", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.after_delete = pipeline;
        Ok(())
    });

    namespace.define_model_decorator("canRead", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.can_read = pipeline;
        Ok(())
    });

    namespace.define_model_decorator("canMutate", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.can_mutate = pipeline;
        Ok(())
    });

    namespace.define_model_decorator("action", |arguments, model| {
        let enable: Result<Value> = arguments.get("enable");
        let disable: Result<Value> = arguments.get("disable");
        if let Ok(enable) = enable {
            if enable.is_array() {
                let mut results = vec![];
                for a in enable.as_array().unwrap() {
                    results.push(a.as_enum_variant().unwrap().value.as_ref().try_into()?);
                }
                model.actions = results;
            } else if enable.is_enum_variant() {
                model.actions = vec![enable.as_enum_variant().unwrap().value.as_ref().try_into()?];
            }
        } else if let Ok(disable) = disable {
            if disable.is_array() {
                let mut results = vec![];
                for a in disable.as_array().unwrap() {
                    let action: Action = a.as_enum_variant().unwrap().value.as_ref().try_into()?;
                    results.push(action.not());
                }
                model.actions = results;
            } else if disable.is_enum_variant() {
                let action: Action = disable.as_enum_variant().unwrap().value.as_ref().try_into()?;
                model.actions = vec![action.not()];
            }
        }
        Ok(())
    });

    namespace.define_model_decorator("generateClient", |arguments, model| {
        let gen: bool = arguments.get("generate")?;
        model.generate_client = gen;
        Ok(())
    });

    namespace.define_model_decorator("generateEntity", |arguments, model| {
        let gen: bool = arguments.get("generate")?;
        model.generate_entity = gen;
        Ok(())
    });

    namespace.define_model_decorator("showInStudio", |arguments, model| {
        let show: bool = arguments.get("show")?;
        model.show_in_studio = show;
        Ok(())
    });
}