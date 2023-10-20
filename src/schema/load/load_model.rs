use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::model::Model;
use crate::model;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::load::load_comment::load_comment;

pub fn load_model(main_namespace: &mut Namespace, schema: &Schema, model_declaration: &teo_parser::ast::model::Model, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut model = Model::new();
    model.path = model_declaration.string_path.clone();
    model.comment = load_comment(model_declaration.comment.as_ref());
    for decorator in &model_declaration.decorators {
        let decorator_declaration = schema.find_top_by_path(&decorator.resolved().path).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.model_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, model_declaration);
            (decorator_implementation.call)(args, &mut model)?;
        }
    }
    for field_declaration in &model_declaration.fields {
        if field_declaration.resolved().class.is_model_primitive_field() {
            model.fields.insert(
                field_declaration.identifier.name().to_owned(),
                load_model_field(field_declaration, schema, diagnostics)?,
            );
        } else if field_declaration.resolved().class.is_model_relation() {
            model.relations.insert(
                field_declaration.identifier.name().to_owned(),
                load_model_relation(field_declaration, schema, diagnostics)?,
            );
        } else if field_declaration.resolved().class.is_model_property() {
            model.properties.insert(
                field_declaration.identifier.name().to_owned(),
                load_model_property(field_declaration, schema, diagnostics)?,
            );
        }
    }
    model.finalize();
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&model_declaration.namespace_str_path());
    dest_namespace.models.insert(model_declaration.identifier.name().to_owned(), model);
    Ok(())
}

fn load_model_field(field_declaration: &teo_parser::ast::field::Field, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<model::Field> {
    unreachable!()
}

fn load_model_relation(field_declaration: &teo_parser::ast::field::Field, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<model::Relation> {
    unreachable!()
}

fn load_model_property(field_declaration: &teo_parser::ast::field::Field, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<model::Property> {
    unreachable!()
}