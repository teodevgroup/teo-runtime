use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::middleware::{Block, Use};
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_argument_list::{fetch_argument_list, fetch_argument_list_or_empty};

pub(super) fn load_use_middlewares(main_namespace: &mut Namespace, schema: &Schema) -> Result<()> {
    for path in &schema.references.use_middlewares_blocks {
        let use_middlewares_block = schema.find_top_by_path(&path).unwrap().as_use_middlewares_block().unwrap();
        let mut block = Block { uses: vec![] };
        for expression in &use_middlewares_block.array_literal.expressions {
            if expression.resolved().value.as_ref().unwrap().is_array() {
                let mut r#use = Use {
                    path: vec![],
                    arguments: Arguments::default(),
                };
                let array = expression.resolved().value.as_ref().unwrap();
                let path: Vec<&str> = array.as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
                if let Some(middleware) = main_namespace.middleware_at_path(&path) {
                    r#use.path = middleware.path.clone();
                }
                if expression.kind.is_unit() {
                    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&use_middlewares_block.namespace_str_path());
                    let last_expression = expression.kind.as_unit().unwrap().expressions.last().unwrap();
                    if let Some(argument_list) = last_expression.kind.as_argument_list() {
                        let arguments = fetch_argument_list(argument_list, schema, use_middlewares_block, dest_namespace)?;
                        r#use.arguments = arguments;
                    }
                }
                if !r#use.path.is_empty() {
                    block.uses.push(r#use);
                }
            }
        }
        let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&use_middlewares_block.namespace_str_path());
        dest_namespace.middlewares_block = Some(block);
    }
    Ok(())
}