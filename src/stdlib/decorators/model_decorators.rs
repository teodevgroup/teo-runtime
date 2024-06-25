use std::ops::Not;
use crate::value::Value;
use teo_result::Result;
use crate::action::Action;
use crate::namespace;
use crate::pipeline::pipeline::Pipeline;
use crate::stdlib::decorators::model_indexable_decorators::{model_id_decorator, model_index_decorator, model_unique_decorator};

pub(in crate::stdlib) fn load_model_decorators(namespace_builder: &namespace::Builder) {

    namespace_builder.define_model_decorator("map", |arguments, model| {
        let table_name: String = arguments.get("tableName")?;
        let mut namespace_prefix = model.namespace_path().join("_");
        if !namespace_prefix.is_empty() {
            namespace_prefix += "__";
        }
        model.set_table_name(namespace_prefix + &table_name);
        Ok(())
    });

    namespace_builder.define_model_decorator("id", |arguments, model| {
        model_id_decorator(arguments, model)
    });

    namespace_builder.define_model_decorator("index", |arguments, model| {
        model_index_decorator(arguments, model)
    });

    namespace_builder.define_model_decorator("unique", |arguments, model| {
        model_unique_decorator(arguments, model)
    });

    namespace_builder.define_model_decorator("migration", |arguments, model| {
        let table_name: Result<Value> = arguments.get("renamed");
        let version: Result<String> = arguments.get("version");
        let drop: Result<bool> = arguments.get("drop");
        let mut migration = model.migration();
        if let Ok(table_name) = table_name {
            if table_name.is_string() {
                migration.renamed = Some(vec![table_name.as_str().unwrap().to_owned()]);
            } else if table_name.is_array() {
                migration.renamed = Some(table_name.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_owned()).collect());
            }
        }
        if let Ok(version) = version {
            migration.version = Some(version);
        }
        if let Ok(drop) = drop {
            migration.drop = drop;
        } else {
            migration.drop = false;
        }
        model.set_migration(migration);
        Ok(())
    });

    namespace_builder.define_model_decorator("beforeSave", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.set_before_save(pipeline);
        Ok(())
    });

    namespace_builder.define_model_decorator("afterSave", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.set_after_save(pipeline);
        Ok(())
    });

    namespace_builder.define_model_decorator("beforeDelete", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.set_before_delete(pipeline);
        Ok(())
    });

    namespace_builder.define_model_decorator("afterDelete", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.set_after_delete(pipeline);
        Ok(())
    });

    namespace_builder.define_model_decorator("canRead", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.set_can_read(pipeline);
        Ok(())
    });

    namespace_builder.define_model_decorator("canMutate", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.set_can_mutate(pipeline);
        Ok(())
    });

    namespace_builder.define_model_decorator("action", |arguments, model| {
        let enable: Result<Value> = arguments.get("enable");
        let disable: Result<Value> = arguments.get("disable");
        if let Ok(enable) = enable {
            if enable.is_array() {
                let mut results = vec![];
                for a in enable.as_array().unwrap() {
                    results.push(a.as_option_variant().unwrap().value.into());
                }
                model.set_actions(results);
            } else if enable.is_option_variant() {
                model.set_actions(vec![enable.as_option_variant().unwrap().value.into()]);
            }
        } else if let Ok(disable) = disable {
            if disable.is_array() {
                let mut results = vec![];
                for a in disable.as_array().unwrap() {
                    let action: Action = a.as_option_variant().unwrap().value.into();
                    results.push(action.not());
                }
                model.set_actions(results);
            } else if disable.is_option_variant() {
                let action: Action = disable.as_option_variant().unwrap().value.into();
                model.set_actions(vec![action.not()]);
            }
        }
        Ok(())
    });

    namespace_builder.define_model_decorator("generateClient", |arguments, model| {
        let gen: bool = arguments.get("generate")?;
        model.set_generate_client(gen);
        Ok(())
    });

    namespace_builder.define_model_decorator("generateEntity", |arguments, model| {
        let gen: bool = arguments.get("generate")?;
        model.set_generate_entity(gen);
        Ok(())
    });

    namespace_builder.define_model_decorator("showInStudio", |arguments, model| {
        let show: bool = arguments.get("show")?;
        model.set_show_in_studio(show);
        Ok(())
    });

    namespace_builder.define_model_decorator("synthesizeShapes", |arguments, model| {
        let synthesize: bool = arguments.get("synthesize")?;
        model.set_synthesize_shapes(synthesize);
        Ok(())
    });
}