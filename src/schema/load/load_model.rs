use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::identifiable::Identifiable;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use crate::model::Field;
use crate::{model, namespace};
use teo_result::Result;
use crate::database::database::Database;
use crate::model::field::is_optional::IsOptional;
use crate::model::field::set_optional::SetOptional;
use crate::model::relation::delete::Delete;
use crate::model::relation::update::Update;
use crate::optionality::Optionality;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::load::load_comment::load_comment;
use crate::schema::load::load_handler::load_handler;
use crate::schema::load::load_handler_inclusion::load_handler_inclusion;

pub fn load_model(main_namespace: &namespace::Builder, schema: &Schema, model_declaration: &teo_parser::ast::model::Model, diagnostics: &mut Diagnostics) -> Result<()> {
    let model_builder = model::Builder::new(
        model_declaration.string_path().clone(),
        model_declaration.path().clone(),
        load_comment(model_declaration.comment()),
    );
    for decorator in model_declaration.decorators() {
        if let Some(decorator_declaration) = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration() {
            if let Some(decorator_implementation) = main_namespace.model_decorator_at_path(&decorator_declaration.str_path()) {
                let args = fetch_decorator_arguments(decorator, schema, model_declaration, main_namespace, diagnostics)?;
                decorator_implementation.call().call(args, &model_builder)?;
            }
        }
    }
    let database = main_namespace.namespace_or_create_at_path(&model_declaration.namespace_str_path()).database();
    for field_declaration in model_declaration.fields() {
        if field_declaration.resolved().class.is_model_primitive_field() {
            if field_declaration.is_available() {
                model_builder.insert_field(
                    field_declaration.identifier().name().to_owned(),
                    load_model_field(main_namespace, field_declaration, schema, database, diagnostics)?,
                );
            }
        } else if field_declaration.resolved().class.is_model_relation() {
            if field_declaration.is_available() {
                model_builder.insert_relation(
                    field_declaration.identifier().name().to_owned(),
                    load_model_relation(main_namespace, field_declaration, schema, database, model_builder.fields().values().collect(), diagnostics)?,
                );
            }
        } else if field_declaration.resolved().class.is_model_property() {
            if field_declaration.is_available() {
                model_builder.insert_property(
                    field_declaration.identifier().name().to_owned(),
                    load_model_property(main_namespace, field_declaration, schema, database, diagnostics)?,
                );
            }
        }
    }
    let mut model = model_builder.build()?;
    model.cache().shape = model_declaration.resolved().clone();
    let dest_namespace = main_namespace.namespace_or_create_at_path(&model_declaration.namespace_str_path());
    dest_namespace.insert_model(model_declaration.identifier().name().to_owned(), model);
    for handler_declaration in model_declaration.handlers() {
        load_handler(main_namespace, schema, handler_declaration, diagnostics)?;
    }
    for handler_inclusion in model_declaration.handler_inclusions() {
        load_handler_inclusion(main_namespace, schema, handler_inclusion, diagnostics)?;
    }
    Ok(())
}

fn load_model_field(main_namespace: &namespace::Builder, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, database: Option<Database>, diagnostics: &mut Diagnostics) -> Result<model::Field> {
    let field_builder = model::field::Builder::new(
        field_declaration.identifier().name().to_owned(),
        load_comment(field_declaration.comment()),
        field_declaration.type_expr().resolved().clone(),
        field_declaration.availability()
    );
    if field_declaration.type_expr().resolved().is_optional() {
        field_builder.set_optional();
    } else {
        field_builder.set_required();
    }
    for decorator in field_declaration.decorators() {
        if let Some(decorator_declaration) = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration() {
            if let Some(decorator_implementation) = main_namespace.model_field_decorator_at_path(&decorator_declaration.str_path()) {
                let args = fetch_decorator_arguments(decorator, schema, field_declaration, main_namespace, diagnostics)?;
                decorator_implementation.call().call(args, &field_builder)?;
            }
        }
    }
    Ok(field_builder.build(database.unwrap(), schema))
}

fn load_model_relation(main_namespace: &namespace::Builder, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, _database: Option<Database>, fields: Vec<Field>, diagnostics: &mut Diagnostics) -> Result<model::Relation> {
    let mut r#type = field_declaration.type_expr().resolved();
    let relation_builder = model::relation::Builder::new(
        field_declaration.identifier().name().to_owned(),
        load_comment(field_declaration.comment()),
        r#type.clone(),
    );
    if r#type.is_optional() {
        relation_builder.set_optionality(Optionality::Optional);
    } else {
        relation_builder.set_optionality(Optionality::Required);
    }
    // set default delete rule
    if r#type.is_optional() {
        relation_builder.set_delete(Delete::Nullify);
    } else if r#type.is_array() {
        relation_builder.set_delete(Delete::NoAction);
    } else {
        relation_builder.set_delete(Delete::Cascade);
    }
    // set default update rule
    if r#type.is_optional() {
        relation_builder.set_update(Update::Nullify);
    } else if r#type.is_array() {
        relation_builder.set_update(Update::NoAction);
    } else {
        relation_builder.set_update(Update::Update);
    }
    r#type = r#type.unwrap_optional();
    relation_builder.set_is_vec(r#type.is_array());
    r#type = r#type.unwrap_array();
    relation_builder.set_model(r#type.as_model_object().unwrap().string_path().clone());
    for decorator in field_declaration.decorators() {
        if let Some(decorator_declaration) = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration() {
            if let Some(decorator_implementation) = main_namespace.model_relation_decorator_at_path(&decorator_declaration.str_path()) {
                let args = fetch_decorator_arguments(decorator, schema, field_declaration, main_namespace, diagnostics)?;
                decorator_implementation.call().call(args, &relation_builder)?;
            }
        }
    }
    Ok(relation_builder.build(fields))
}

fn load_model_property(main_namespace: &namespace::Builder, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, database: Option<Database>, diagnostics: &mut Diagnostics) -> Result<model::Property> {
    let property_builder = model::property::Builder::new(
        field_declaration.identifier().name().to_owned(),
        load_comment(field_declaration.comment()),
        field_declaration.type_expr().resolved().clone(),
    );
    let r#type = field_declaration.type_expr().resolved();
    if r#type.is_optional() {
        property_builder.set_optional();
    } else {
        property_builder.set_required();
    }
    for decorator in field_declaration.decorators() {
        if let Some(decorator_declaration) = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration() {
            if let Some(decorator_implementation) = main_namespace.model_property_decorator_at_path(&decorator_declaration.str_path()) {
                let args = fetch_decorator_arguments(decorator, schema, field_declaration, main_namespace, diagnostics)?;
                decorator_implementation.call().call(args, &property_builder)?;
            }
        }
    }
    Ok(property_builder.build(database.unwrap(), schema))
}