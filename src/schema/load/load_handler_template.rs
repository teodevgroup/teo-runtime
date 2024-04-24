use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::handler::Handler;
use crate::handler::handler::Method;
use crate::namespace::Namespace;
use teo_result::Error;
use crate::request;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;


pub fn load_handler_template(main_namespace: &mut Namespace, schema: &Schema, handler_template_declaration: &teo_parser::ast::handler_template_declaration::HandlerTemplateDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut handler = if let Some(handler) = main_namespace.handler_template_at_path(&handler_template_declaration.str_path()).cloned() {
        handler
    } else {
        // just load a default empty one, for generating interfaces
        Handler {
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            nonapi: false,
            format: HandlerInputFormat::Json,
            path: handler_template_declaration.string_path().clone(),
            ignore_prefix: false,
            method: Method::Post,
            interface: None,
            url: None,
            namespace_path: handler_template_declaration.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                Err(Error::not_found())
            })),
        }
    };
    handler.format = handler_template_declaration.input_format;
    handler.nonapi = handler_template_declaration.nonapi;
    handler.input_type = handler_template_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone());
    handler.output_type = handler_template_declaration.output_type().resolved().clone();
    for decorator in handler_template_declaration.decorators() {
        let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_template_declaration, main_namespace, diagnostics)?;
            decorator_implementation.call.call(args, &mut handler)?;
        }
    }
    main_namespace.replace_handler_template_at_path(&handler_template_declaration.str_path(), handler);
    Ok(())
}