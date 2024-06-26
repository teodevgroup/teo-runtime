use crate::namespace;

pub(super) fn load_admin_library(std_namespace: &namespace::Builder) {

    let mut admin_namespace = std_namespace.namespace_or_create("admin");

    admin_namespace.define_model_decorator("administrator", |_, model| {
        model.insert_data_entry("admin:administrator".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_decorator("ignore", |_, model| {
        model.insert_data_entry("admin:ignore".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("title", |_, field| {
        field.insert_data_entry("admin:title".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("subtitle", |_, field| {
        field.insert_data_entry("admin:subtitle".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("coverImage", |_, field| {
        field.insert_data_entry("admin:coverImage".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_field_decorator("secureInput", |_, field| {
        field.insert_data_entry("admin:secureInput".to_owned(), true.into());
        Ok(())
    });

    admin_namespace.define_model_relation_decorator("embedded", |_, field| {
        field.insert_data_entry("admin:embedded".to_owned(), true.into());
        Ok(())
    });
}

