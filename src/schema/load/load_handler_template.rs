use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use teo_result::Error;
use crate::{handler, namespace};
use crate::request::Request;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;


pub fn load_handler_template(main_namespace: &namespace::Builder, schema: &Schema, handler_template_declaration: &teo_parser::ast::handler_template_declaration::HandlerTemplateDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let handler_builder = if let Some(handler) = main_namespace.handler_template_at_path(handler_template_declaration.string_path()) {
        handler::Builder::new(
            handler.path().clone(),
            handler.namespace_path().clone(),
            handler_template_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone()),
            handler_template_declaration.output_type().resolved().clone(),
            handler_template_declaration.nonapi,
            handler_template_declaration.input_format,
            handler.call(),
            main_namespace.app_data().clone(),
        )
    } else {
        // just load a default empty one, for generating interfaces
        handler::Builder::new(
            handler_template_declaration.string_path().clone(),
            handler_template_declaration.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            handler_template_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone()),
            handler_template_declaration.output_type().resolved().clone(),
            handler_template_declaration.nonapi,
            handler_template_declaration.input_format,
            Box::leak(Box::new(|request: Request| async {
                Err(Error::not_found())
            })),
            main_namespace.app_data().clone(),
        )
    };
    for decorator in handler_template_declaration.decorators() {
        let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_template_declaration, main_namespace, diagnostics)?;
            decorator_implementation.call().call(args, &handler_builder)?;
        }
    }
    main_namespace.replace_handler_template_at_path(&handler_template_declaration.str_path(), handler_builder.build());
    Ok(())
}