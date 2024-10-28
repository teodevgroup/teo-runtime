use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::handler::Method;
use teo_result::Error;
use crate::{handler, namespace, request};
use crate::request::Request;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;


pub fn load_handler(main_namespace: &namespace::Builder, schema: &Schema, handler_declaration: &teo_parser::ast::handler::HandlerDeclaration, diagnostics: &mut Diagnostics) -> Result<()> {
    let handler_builder = if let Some(handler) = main_namespace.handler_at_path(&handler_declaration.str_path()) {
        handler::Builder::new(
            handler.path().clone(),
            handler.namespace_path().clone(),
            handler_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone()),
            handler_declaration.output_type().resolved().clone(),
            handler_declaration.nonapi,
            handler_declaration.input_format,
            handler.call(),
            main_namespace.app_data().clone(),
        )
    } else {
        // just load a default empty one, for generating interfaces
        handler::Builder::new(
            handler_declaration.string_path().clone(),
            handler_declaration.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            handler_declaration.input_type().map_or(Type::Any, |t| t.resolved().clone()),
            handler_declaration.output_type().resolved().clone(),
            handler_declaration.nonapi,
            handler_declaration.input_format,
            Box::leak(Box::new(|request: Request| async {
                Err(Error::not_found())
            })),
            main_namespace.app_data().clone(),
        )
    };
    for decorator in handler_declaration.decorators() {
        let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_declaration, main_namespace, diagnostics)?;
            decorator_implementation.call().call(args, &handler_builder)?;
        }
    }
    if (handler_builder.method() != Method::Post) || handler_builder.url().is_some() {
        let parent_string_path = handler_declaration.parent_string_path();
        main_namespace.handler_map().lock().unwrap().add_record(
            &handler_declaration.namespace_str_path(),
            if handler_declaration.namespace_skip() == 2 {
                Some(parent_string_path.last().unwrap().as_str())
            } else {
                None
            },
            handler_declaration.name(),
            handler_builder.method(),
            handler_builder.url().as_ref().map(|u| u.as_str()),
            handler_builder.ignore_prefix(),
        );
    }
    main_namespace.replace_handler_at_path(&handler_declaration.str_path(), handler_builder.build(), handler_declaration.inside_group);
    Ok(())
}