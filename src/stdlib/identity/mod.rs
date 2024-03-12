use crate::namespace::Namespace;
use crate::pipeline::Pipeline;

pub(super) fn load_identity_library(std_namespace: &mut Namespace) {

    let mut identity_namespace = std_namespace.namespace_mut_or_create("identity");

    identity_namespace.define_model_field_decorator("id", |arguments, field| {
        field.data.insert("identity:id".to_owned(), true.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("checker", |arguments, field| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        field.data.insert("identity:checker".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("companion", |arguments, field| {
        field.data.insert("identity:companion".to_owned(), true.into());
        Ok(())
    });
}

