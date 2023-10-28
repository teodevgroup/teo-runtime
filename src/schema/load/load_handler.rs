use teo_parser::ast::identifiable::Identifiable;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;

pub fn load_handler(main_namespace: &mut Namespace, schema: &Schema, handler_declaration: &teo_parser::ast::handler::HandlerDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut handler = main_namespace.handler_at_path(&handler_declaration.str_path()).unwrap().clone();
    handler.format = handler_declaration.input_format;
    for decorator in &handler_declaration.decorators {
        let decorator_declaration = schema.find_top_by_path(&decorator.resolved().path).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_declaration, main_namespace)?;
            (decorator_implementation.call)(args, &mut handler)?;
        }
    }
    main_namespace.replace_handler_at_path(&handler_declaration.str_path(), handler);
    Ok(())
}