use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::interface;
use crate::interface::Interface;
use crate::model::field::is_optional::{IsOptional};
use crate::schema::load::load_comment::load_comment;

pub fn load_interface(main_namespace: &mut Namespace, schema: &Schema, interface_declaration: &teo_parser::ast::interface::InterfaceDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut interface = Interface::new();
    interface.path = interface_declaration.string_path.clone();
    interface.comment = load_comment(interface_declaration.comment.as_ref());
    for field_declaration in &interface_declaration.fields {
        if field_declaration.is_available() {
            interface.fields.insert(
                field_declaration.identifier.name().to_owned(),
                load_interface_field(main_namespace, field_declaration, schema, diagnostics)?,
            );
        }
    }
    Ok(())
}

fn load_interface_field(main_namespace: &mut Namespace, field_declaration: &teo_parser::ast::field::Field, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<interface::Field> {
    let mut field = interface::Field::new();
    field.name = field_declaration.identifier.name().to_owned();
    field.comment = load_comment(field_declaration.comment.as_ref());
    if field_declaration.type_expr.resolved().is_optional() {
        field.set_optional();
    } else {
        field.set_required();
    }
    field.r#type = field_declaration.type_expr.resolved().unwrap_optional().clone();
    Ok(field)
}