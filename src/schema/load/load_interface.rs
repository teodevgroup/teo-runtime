use indexmap::indexmap;
use maplit::btreemap;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::identifiable::Identifiable;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::{interface, namespace};
use crate::interface::Interface;
use crate::model::field::is_optional::{IsOptional};
use crate::model::field::set_optional::SetOptional;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::load::load_comment::load_comment;

pub fn load_interface(main_namespace_builder: &namespace::Builder, schema: &Schema, interface_declaration: &teo_parser::ast::interface::InterfaceDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut fields = indexmap! {};
    let mut generic_names = vec![];
    let mut extends = vec![];
    if let Some(generics_declaration) = interface_declaration.generics_declaration() {
        for gen in generics_declaration.identifiers() {
            generic_names.push(gen.name().to_owned().clone());
        }
    }
    for t in interface_declaration.extends() {
        extends.push(t.resolved().clone());
    }
    for field_declaration in interface_declaration.fields() {
        if field_declaration.is_available() {
            fields.insert(
                field_declaration.identifier().name().to_owned(),
                load_interface_field(main_namespace_builder, field_declaration, schema, diagnostics)?,
            );
        }
    }
    let interface_builder = interface::Builder::new(
        interface_declaration.string_path().clone(),
        interface_declaration.path().clone(),
        load_comment(interface_declaration.comment()),
        fields,
        generic_names,
        extends,
        interface_declaration.resolved().shape().clone(),
    );
    for decorator in interface_declaration.decorators() {
        if let Some(decorator_declaration) = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration() {
            if let Some(decorator_implementation) = main_namespace_builder.interface_decorator_at_path(&decorator_declaration.str_path()) {
                let args = fetch_decorator_arguments(decorator, schema, interface_declaration, main_namespace_builder, diagnostics)?;
                decorator_implementation.call.call(args, &interface_builder)?;
            }
        }
    }
    let dest_namespace = main_namespace_builder.namespace_or_create_at_path(&interface_declaration.namespace_string_path());
    dest_namespace.insert_interface(interface_declaration.identifier().name().to_owned(), interface_builder.build());
    Ok(())
}

fn load_interface_field(main_namespace_builder: &namespace::Builder, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<interface::Field> {
    let field_builder = interface::field::Builder::new(
        field_declaration.identifier().name().to_owned(),
        load_comment(field_declaration.comment()),
        field_declaration.type_expr().resolved().clone(),
    );
    if field_declaration.type_expr().resolved().is_optional() {
        field_builder.set_optional();
    } else {
        field_builder.set_required();
    }
    Ok(field_builder.build())
}