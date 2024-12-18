use teo_parser::ast::schema::Schema;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use teo_parser::ast::arith_expr::ArithExpr;
use teo_parser::ast::middleware::MiddlewareType;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::arguments::Arguments;
use crate::middleware::{Middleware, Use};
use crate::middleware::middleware_imp::{combine_middleware, empty_middleware, MiddlewareImp};
use crate::{middleware, namespace};
use crate::schema::fetch::fetch_argument_list::{fetch_argument_list};

pub(super) fn load_use_middlewares(main_namespace: &namespace::Builder, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<()> {
    // load middleware blocks
    for path in &schema.references.use_middlewares_blocks {
        let use_middlewares_block = schema.find_top_by_path(&path).unwrap().as_use_middlewares_block().unwrap();
        let mut uses = vec![];
        for expression in use_middlewares_block.array_literal().expressions() {
            if let Some(reference_info) = expression.resolved().reference_info() {
                let path = reference_info.reference.str_path();
                let mut arguments = Arguments::default();
                if let Some(middleware) = main_namespace.middleware_at_path_with_type(&path, use_middlewares_block.middleware_type()) {
                    let creator = middleware.creator();
                    if let Some(arith_expr) = expression.kind.as_arith_expr() {
                        match arith_expr {
                            ArithExpr::Expression(expression) => {
                                if expression.kind.is_unit() {
                                    let dest_namespace = main_namespace.descendant_namespace_or_create_at_path(&use_middlewares_block.namespace_string_path());
                                    let last_expression = expression.kind.as_unit().unwrap().expression_at(expression.kind.as_unit().unwrap().expressions().count() - 1).unwrap();
                                    if let Some(argument_list) = last_expression.kind.as_argument_list() {
                                        let new_arguments = fetch_argument_list(argument_list, schema, use_middlewares_block, &dest_namespace, diagnostics)?;
                                        arguments = new_arguments;
                                    }
                                }
                            },
                            _ => ()
                        }
                    }
                    uses.push(Use::new(path.iter().map(|s| s.to_string()).collect(), creator, arguments));
                }
            }
        }
        let dest_namespace = main_namespace.descendant_namespace_or_create_at_path(&use_middlewares_block.namespace_string_path());
        match use_middlewares_block.middleware_type() {
            MiddlewareType::HandlerMiddleware => {
                dest_namespace.set_handler_middlewares_block(Some(middleware::Block::new(uses)));
            },
            MiddlewareType::RequestMiddleware => {
                dest_namespace.set_request_middlewares_block(Some(middleware::Block::new(uses)));
            }
        }
    }

    // load middleware stack
    load_middleware_stack(main_namespace, empty_middleware(), empty_middleware())?;
    Ok(())
}

fn load_middleware_stack(namespace: &namespace::Builder, parent_handler_stack: Middleware, parent_request_stack: Middleware) -> Result<()> {
    if let Some(block) = namespace.handler_middlewares_block() {
        let mut middlewares = vec![];
        middlewares.push(parent_handler_stack);
        for r#use in block.uses() {
            let middleware = r#use.creator().call(r#use.arguments().clone())?;
            middlewares.push(middleware);
        }
        middlewares.reverse();
        namespace.set_handler_middleware_stack(combine_middleware(middlewares));
    } else {
        namespace.set_handler_middleware_stack(parent_handler_stack);
    }
    if let Some(block) = namespace.request_middlewares_block() {
        let mut middlewares = vec![];
        middlewares.push(parent_request_stack);
        for r#use in block.uses() {
            let middleware = r#use.creator().call(r#use.arguments().clone())?;
            middlewares.push(middleware);
        }
        middlewares.reverse();
        namespace.set_request_middleware_stack(combine_middleware(middlewares));
    } else {
        namespace.set_request_middleware_stack(parent_request_stack);
    }
    for child_namespace in namespace.namespaces().values() {
        load_middleware_stack(child_namespace, namespace.handler_middleware_stack(), namespace.request_middleware_stack())?;
    }
    Ok(())
}