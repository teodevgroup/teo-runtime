use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::pipeline::pipeline::Pipeline;
use crate::stdlib::decorators::indexable_decorators::{id_decorator, index_decorator, unique_decorator};

pub(in crate::stdlib) fn load_model_property_decorators(namespace: &mut Namespace) {

    namespace.define_model_property_decorator("getter", |arguments, property| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        property.getter = Some(pipeline);
        Ok(())
    });

    namespace.define_model_property_decorator("setter", |arguments, property| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        property.setter = Some(pipeline);
        Ok(())
    });

    namespace.define_model_property_decorator("cached", |arguments, property| {
        property.cached = true;
        Ok(())
    });

    namespace.define_model_property_decorator("deps", |arguments, property| {
        let deps: Value = arguments.get("deps")?;
        let deps: Vec<EnumVariant> = deps.into_vec()?;
        let deps: Vec<String> = deps.iter().map(|f| f.clone().into_string()).collect::<Vec<_>>();
        property.dependencies = deps;
        Ok(())
    });

    namespace.define_model_property_decorator("id", |arguments, property| {
        id_decorator(arguments, property)
    });

    namespace.define_model_property_decorator("index", |arguments, property| {
        index_decorator(arguments, property)
    });

    namespace.define_model_property_decorator("unique", |arguments, property| {
        unique_decorator(arguments, property)
    });
}