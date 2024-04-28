use crate::namespace::Namespace;

pub(super) fn load_admin_library(std_namespace: &mut Namespace) {

    let mut admin_namespace = std_namespace.namespace_mut_or_create("admin");

    admin_namespace.define_model_decorator("administrator", |_, model| {
        model.data.insert("admin:administrator".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_decorator("ignore", |_, model| {
        model.data.insert("admin:ignore".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("title", |_, field| {
        field.data.insert("admin:title".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("subtitle", |_, field| {
        field.data.insert("admin:subtitle".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("coverImage", |_, field| {
        field.data.insert("admin:coverImage".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("secureInput", |_, field| {
        field.data.insert("admin:secureInput".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_relation_decorator("embedded", |_, field| {
        field.data.insert("admin:embedded".to_owned(), true.into());
        Ok(())
    });
}

