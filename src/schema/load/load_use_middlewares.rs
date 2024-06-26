use teo_parser::ast::schema::Schema;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use async_recursion::async_recursion;
use teo_parser::ast::arith_expr::ArithExpr;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::arguments::Arguments;
use crate::middleware::{Block, Use};
use crate::middleware::middleware::{combine_middleware, empty_middleware, Middleware};
use crate::namespace;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_argument_list::{fetch_argument_list, fetch_argument_list_or_empty};

pub(super) async fn load_use_middlewares(main_namespace: &namespace::Builder, schema: &Schema, diagnostics: &mut Diagnostics) -> Result<()> {
    // load middleware blocks
    for path in &schema.references.use_middlewares_blocks {
        let use_middlewares_block = schema.find_top_by_path(&path).unwrap().as_use_middlewares_block().unwrap();
        let mut block = Block { uses: vec![] };
        for expression in use_middlewares_block.array_literal().expressions() {
            if let Some(reference_info) = expression.resolved().reference_info() {
                let path = reference_info.reference.str_path();
                let mut arguments = Arguments::default();
                if let Some(middleware) = main_namespace.middleware_at_path(&path) {
                    let creator = middleware.creator();
                    if let Some(arith_expr) = expression.kind.as_arith_expr() {
                        match arith_expr {
                            ArithExpr::Expression(expression) => {
                                if expression.kind.is_unit() {
                                    let dest_namespace = main_namespace.namespace_or_create_at_path(&use_middlewares_block.namespace_str_path());
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
                    block.uses.push(Use::new(path.iter().map(|s| s.to_string()).collect(), creator, arguments));
                }
            }
        }
        let dest_namespace = main_namespace.namespace_or_create_at_path(&use_middlewares_block.namespace_str_path());
        dest_namespace.set_middlewares_block(Some(block));
    }

    // load middleware stack
    load_middleware_stack(main_namespace, empty_middleware()).await?;
    Ok(())
}

#[async_recursion]
async fn load_middleware_stack(namespace: &mut Namespace, parent_stack: &'static dyn Middleware) -> Result<()> {
    if let Some(block) = &namespace.middlewares_block {
        let mut middlewares = vec![];
        middlewares.push(parent_stack);
        for r#use in &block.uses {
            let middleware = r#use.creator().call(r#use.arguments().clone()).await?;
            middlewares.push(middleware);
        }
        middlewares.reverse();
        namespace.middleware_stack = combine_middleware(middlewares);
    } else {
        namespace.middleware_stack = parent_stack;
    }
    for child_namespace in namespace.namespaces.values_mut() {
        load_middleware_stack(child_namespace, namespace.middleware_stack).await?;
    }
    Ok(())
}