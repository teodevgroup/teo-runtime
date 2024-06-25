use crate::namespace;
use crate::value::Value;
use crate::pipeline::pipeline::Pipeline;
use crate::stdlib::decorators::indexable_decorators::{id_decorator, index_decorator, unique_decorator};

pub(in crate::stdlib) fn load_model_property_decorators(namespace: &namespace::Builder) {

    namespace.define_model_property_decorator("getter", |arguments, property| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        property.set_getter(Some(pipeline));
        Ok(())
    });

    namespace.define_model_property_decorator("setter", |arguments, property| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        property.set_setter(Some(pipeline));
        Ok(())
    });

    namespace.define_model_property_decorator("cached", |arguments, property| {
        property.set_cached(true);
        Ok(())
    });

    namespace.define_model_property_decorator("deps", |arguments, property| {
        let deps: Value = arguments.get("deps")?;
        let deps: Vec<String> = deps.wrap_into_vec()?;
        property.set_dependencies(deps);
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

    namespace.define_model_property_decorator("inputOmissible", |arguments, property| {
        property.set_input_omissible(true);
        Ok(())
    });

    namespace.define_model_property_decorator("outputOmissible", |arguments, property| {
        property.set_output_omissible(true);
        Ok(())
    });
}