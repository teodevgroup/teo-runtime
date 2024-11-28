use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::ast::include_handler_from_template::IncludeHandlerFromTemplate;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use hyper::Method;
use teo_result::Error;
use crate::{handler, namespace};
use crate::request::Request;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;


pub fn load_handler_inclusion(main_namespace: &namespace::Builder, schema: &Schema, handler_inclusion: &IncludeHandlerFromTemplate, diagnostics: &mut Diagnostics) -> Result<()> {
    let template_path = &handler_inclusion.resolved().template_path;
    let handler_builder = if let Some(handler) = main_namespace.handler_template_at_path(template_path) {
        handler::Builder::new(
            handler_inclusion.string_path().clone(),
            handler_inclusion.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            handler_inclusion.resolved().input_type.as_ref().map_or(Type::Any, |t| t.clone()),
            handler_inclusion.resolved().output_type.clone(),
            handler.nonapi(),
            handler.format(),
            handler.call(),
            main_namespace.app_data().clone(),
        )
    } else {
        // just load a default empty one, for generating interfaces
        handler::Builder::new(
            handler_inclusion.string_path().clone(),
            handler_inclusion.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            handler_inclusion.resolved().input_type.as_ref().map_or(Type::Any, |t| t.clone()),
            handler_inclusion.resolved().output_type.clone(),
            false,
            HandlerInputFormat::Json,
            Box::leak(Box::new(|request: Request| async {
                Err(Error::not_found())
            })),
            main_namespace.app_data().clone(),
        )
    };
    for decorator in handler_inclusion.decorators() {
        let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_inclusion, main_namespace, diagnostics)?;
            decorator_implementation.call().call(args, &handler_builder)?;
        }
    }
    if (handler_builder.method() != Method::POST) || handler_builder.url().is_some() {
        let parent_string_path = handler_inclusion.parent_string_path();
        main_namespace.handler_map().lock().unwrap().add_record(
            &handler_inclusion.namespace_str_path(),
            if handler_inclusion.namespace_skip() == 2 {
                Some(parent_string_path.last().unwrap().as_str())
            } else {
                None
            },
            handler_inclusion.name(),
            handler_builder.method(),
            handler_builder.url().as_ref().map(AsRef::as_ref),
            handler_builder.ignore_prefix(),
        );
    }
    if (handler_builder.method() != Method::POST) || handler_builder.url().is_some() {
        let parent_string_path = handler_inclusion.parent_string_path();
        main_namespace.handler_map().lock().unwrap().add_record(
            &handler_inclusion.namespace_str_path(),
            if handler_inclusion.namespace_skip() == 2 {
                Some(parent_string_path.last().unwrap().as_str())
            } else {
                None
            },
            handler_inclusion.name(),
            handler_builder.method(),
            handler_builder.url().as_ref().map(|u| u.as_str()),
            handler_builder.ignore_prefix(),
        );
    }
    main_namespace.replace_handler_at_path(&handler_inclusion.str_path(), handler_builder.build(), true)?;
    Ok(())
}