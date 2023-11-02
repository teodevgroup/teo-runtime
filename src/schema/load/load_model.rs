use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::model::{Field, Model};
use crate::model;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::database::database::Database;
use crate::model::field::is_optional::IsOptional;
use crate::optionality::Optionality;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::load::load_comment::load_comment;
use crate::schema::load::load_handler::load_handler;

pub fn load_model(main_namespace: &mut Namespace, schema: &Schema, model_declaration: &teo_parser::ast::model::Model, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut model = Model::new();
    model.path = model_declaration.string_path.clone();
    model.comment = load_comment(model_declaration.comment.as_ref());
    for decorator in &model_declaration.decorators {
        let decorator_declaration = schema.find_top_by_path(&decorator.resolved().path).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.model_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, model_declaration, main_namespace)?;
            (decorator_implementation.call)(args, &mut model)?;
        }
    }
    let database = main_namespace.namespace_mut_or_create_at_path(&model_declaration.namespace_str_path()).database;
    for field_declaration in &model_declaration.fields {
        if field_declaration.resolved().class.is_model_primitive_field() {
            if field_declaration.is_available() {
                model.fields.insert(
                    field_declaration.identifier.name().to_owned(),
                    load_model_field(main_namespace, field_declaration, schema, database, diagnostics)?,
                );
            }
        } else if field_declaration.resolved().class.is_model_relation() {
            if field_declaration.is_available() {
                model.relations.insert(
                    field_declaration.identifier.name().to_owned(),
                    load_model_relation(main_namespace, field_declaration, schema, database, model.fields.values().collect(), diagnostics)?,
                );
            }
        } else if field_declaration.resolved().class.is_model_property() {
            if field_declaration.is_available() {
                model.properties.insert(
                    field_declaration.identifier.name().to_owned(),
                    load_model_property(main_namespace, field_declaration, schema, database, diagnostics)?,
                );
            }
        }
    }
    model.finalize()?;
    model.cache.shape = model_declaration.shape_resolved().clone();
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&model_declaration.namespace_str_path());
    dest_namespace.models.insert(model_declaration.identifier.name().to_owned(), model);
    for handler_declaration in &model_declaration.handlers {
        load_handler(main_namespace, schema, handler_declaration, diagnostics)?;
    }
    Ok(())
}

fn load_model_field(main_namespace: &mut Namespace, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, database: Option<Database>, diagnostics: &mut Diagnostics) -> Result<model::Field> {
    let mut field = model::Field::new();
    field.name = field_declaration.identifier.name().to_owned();
    field.comment = load_comment(field_declaration.comment.as_ref());
    if field_declaration.type_expr.resolved().is_optional() {
        field.set_optional();
    } else {
        field.set_required();
    }
    field.r#type = field_declaration.type_expr.resolved().unwrap_optional().clone();
    for decorator in &field_declaration.decorators {
        let decorator_declaration = schema.find_top_by_path(&decorator.resolved().path).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.model_field_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, field_declaration, main_namespace)?;
            (decorator_implementation.call)(args, &mut field)?;
        }
    }
    field.finalize(database.unwrap())?;
    Ok(field)
}

fn load_model_relation(main_namespace: &mut Namespace, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, database: Option<Database>, fields: Vec<&Field>, diagnostics: &mut Diagnostics) -> Result<model::Relation> {
    let mut relation = model::Relation::new();
    relation.name = field_declaration.identifier.name().to_owned();
    relation.comment = load_comment(field_declaration.comment.as_ref());
    let mut r#type = field_declaration.type_expr.resolved();
    relation.r#type = r#type.clone();
    if r#type.is_optional() {
        relation.optionality = Optionality::Optional;
    } else {
        relation.optionality = Optionality::Required;
    }
    r#type = r#type.unwrap_optional();
    relation.is_vec = r#type.is_array();
    r#type = r#type.unwrap_array();
    relation.model = r#type.as_model_object().unwrap().1.clone();
    for decorator in &field_declaration.decorators {
        let decorator_declaration = schema.find_top_by_path(&decorator.resolved().path).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.model_relation_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, field_declaration, main_namespace)?;
            (decorator_implementation.call)(args, &mut relation)?;
        }
    }
    relation.finalize(database.unwrap(), fields);
    Ok(relation)
}

fn load_model_property(main_namespace: &mut Namespace, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, database: Option<Database>, diagnostics: &mut Diagnostics) -> Result<model::Property> {
    let mut property = model::Property::new();
    property.name = field_declaration.identifier.name().to_owned();
    property.comment = load_comment(field_declaration.comment.as_ref());
    let mut r#type = field_declaration.type_expr.resolved();
    if r#type.is_optional() {
        property.optionality = Optionality::Optional;
    } else {
        property.optionality = Optionality::Required;
    }
    r#type = r#type.unwrap_optional();
    property.r#type = r#type.clone();
    for decorator in &field_declaration.decorators {
        let decorator_declaration = schema.find_top_by_path(&decorator.resolved().path).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.model_property_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, field_declaration, main_namespace)?;
            (decorator_implementation.call)(args, &mut property)?;
        }
    }
    property.finalize(database.unwrap())?;
    Ok(property)
}