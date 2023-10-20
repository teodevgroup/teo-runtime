use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::pipeline::pipeline::Pipeline;

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
        let deps: Vec<String> = deps.iter().map(|f| f.clone().into_string()).collect::<Result<Vec<_>>>()?;
        property.dependencies = deps;
        Ok(())
    });

    // /// @name Index
    // /// Define index for this cached property
    // declare unique model property decorator index(sort: Sort?, length: Int?, map: String?)
    //
    // /// @name Unique
    // /// Define unique index for this cached property
    // declare unique model property decorator unique(sort: Sort?, length: Int?, map: String?)
}