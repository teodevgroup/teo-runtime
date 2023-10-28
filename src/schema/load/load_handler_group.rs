use teo_parser::ast::identifiable::Identifiable;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::load::load_handler::load_handler;

pub fn load_handler_group(main_namespace: &mut Namespace, schema: &Schema, handler_group_declaration: &teo_parser::ast::handler::HandlerGroupDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    for handler_declaration in &handler_group_declaration.handler_declarations {
        load_handler(main_namespace, schema, handler_declaration, diagnostics)?;
    }
    Ok(())
}


