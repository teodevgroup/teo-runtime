use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::ast::include_handler_from_template::IncludeHandlerFromTemplate;
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


pub fn load_handler_inclusion(main_namespace: &mut Namespace, schema: &Schema, handler_inclusion: &IncludeHandlerFromTemplate, diagnostics: &mut Diagnostics) -> Result<()> {
    let template_path: Vec<&str> = handler_inclusion.resolved().template_path.iter().map(|i| i.as_str()).collect();
    let mut handler = if let Some(handler) = main_namespace.handler_template_at_path(&template_path).cloned() {
        handler
    } else {
        // just load a default empty one, for generating interfaces
        Handler {
            input_type: Type::Undetermined,
            output_type: Type::Undetermined,
            nonapi: false,
            format: HandlerInputFormat::Json,
            path: handler_inclusion.string_path().clone(),
            ignore_prefix: false,
            method: Method::Post,
            interface: None,
            url: None,
            namespace_path: handler_inclusion.namespace_str_path().iter().map(|s| s.to_string()).collect(),
            call: Box::leak(Box::new(|ctx: request::Ctx| async {
                Err(Error::not_found())
            })),
        }
    };
    handler.path = handler_inclusion.string_path().clone();
    handler.namespace_path = handler_inclusion.namespace_str_path().iter().map(|s| s.to_string()).collect();
    handler.input_type = handler_inclusion.resolved().input_type.as_ref().map_or(Type::Any, |t| t.clone());
    handler.output_type = handler_inclusion.resolved().output_type.clone();
    for decorator in handler_inclusion.decorators() {
        let decorator_declaration = schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        if let Some(decorator_implementation) = main_namespace.handler_decorator_at_path(&decorator_declaration.str_path()) {
            let args = fetch_decorator_arguments(decorator, schema, handler_inclusion, main_namespace)?;
            decorator_implementation.call.call(args, &mut handler)?;
        }
    }
    if (handler.method != Method::Post) || handler.url.is_some() {
        let parent_string_path = handler_inclusion.parent_string_path();
        main_namespace.handler_map.add_record(
            &handler_inclusion.namespace_str_path(),
            if handler_inclusion.namespace_skip() == 2 {
                Some(parent_string_path.last().unwrap().as_str())
            } else {
                None
            },
            handler_inclusion.name(),
            handler.method,
            handler.url.as_ref().map(|u| u.as_str()),
            handler.ignore_prefix,
        );
    }
    if (handler.method != Method::Post) || handler.url.is_some() {
        let parent_string_path = handler_inclusion.parent_string_path();
        main_namespace.handler_map.add_record(
            &handler_inclusion.namespace_str_path(),
            if handler_inclusion.namespace_skip() == 2 {
                Some(parent_string_path.last().unwrap().as_str())
            } else {
                None
            },
            handler_inclusion.name(),
            handler.method,
            handler.url.as_ref().map(|u| u.as_str()),
            handler.ignore_prefix,
        );
    }
    main_namespace.replace_handler_at_path(&handler_inclusion.str_path(), handler, true);
    Ok(())
}