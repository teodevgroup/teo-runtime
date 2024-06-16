use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::handler::Handler;
use hyper::Method;
use crate::namespace::Namespace;
use teo_result::Error;
use crate::request;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;


pub fn load_handler(main_namespace: &mut Namespace, schema: &Schema, handler_declaration: &teo_parser::ast::handler::HandlerDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut handler = if let Some(handler) = main_namespace.handler_at_path(&handler_declaration.str_path()).cloned() {
        handler
    } else {
        // just load a default empty one, for generating interfaces
        Handler {
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            nonapi: false,
            format: HandlerInputFormat::Json,
            path: handler_declaration.string_path().clone(),
            ignore_prefix: false,
            method: Method::POST,
            interface: None,
            url: None,
            namespace_path: handler_declaration.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                Err(Error::not_found())
            })),
        }
    };
    handler.format = handler_declaration.input_format;
    handler.nonapi = handler_declaration.nonapi;
    handler.input_type = handler_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone());
    handler.output_type = handler_declaration.output_type().resolved().clone();
    for decorator in handler_declaration.decorators() {
        let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_declaration, main_namespace, diagnostics)?;
            decorator_implementation.call.call(args, &mut handler)?;
        }
    }
    if (handler.method != Method::POST) || handler.url.is_some() {
        let parent_string_path = handler_declaration.parent_string_path();
        main_namespace.handler_map.add_record(
            &handler_declaration.namespace_str_path(),
            if handler_declaration.namespace_skip() == 2 {
                Some(parent_string_path.last().unwrap().as_str())
            } else {
                None
            },
            handler_declaration.name(),
            handler.method.clone(),
            handler.url.as_ref().map(|u| u.as_str()),
            handler.ignore_prefix,
        );
    }
    main_namespace.replace_handler_at_path(&handler_declaration.str_path(), handler, handler_declaration.inside_group);
    Ok(())
}