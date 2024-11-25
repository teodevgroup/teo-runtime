use std::sync::Arc;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::ast::node::Node;
use teo_parser::ast::pipeline::PipelineResolved;
use teo_parser::ast::unit::Unit;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_parser::utils::top_filter::top_filter_for_pipeline;
use teo_result::{Error, Result};
use crate::namespace;
use crate::pipeline::item::BoundedItem;
use crate::pipeline::Pipeline;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list_or_empty;
use crate::schema::fetch::fetchers::fetch_identifier::fetch_identifier_to_node;
use crate::value::Value;

pub fn fetch_pipeline<I>(pipeline: &teo_parser::ast::pipeline::Pipeline, schema: &Schema, info_provider: &I, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    fetch_pipeline_unit(&pipeline.resolved().replace_generics(expect.clone()), pipeline.unit(), schema, info_provider, expect, namespace, diagnostics)
}

fn fetch_pipeline_unit<I>(pipeline_resolved: &PipelineResolved, unit: &Unit, schema: &Schema, info_provider: &I, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    let mut pipeline = Pipeline::new();
    let mut current_space: Option<&teo_parser::ast::namespace::Namespace> = None;
    let mut item_index = 0;
    for (index, expression) in unit.expressions().enumerate() {
        if let Some(identifier) = expression.kind.as_identifier() {
            if let Some(this_top) = if current_space.is_some() {
                current_space.unwrap().find_top_by_name(identifier.name(), &top_filter_for_pipeline(), info_provider.availability())
            } else {
                Some(fetch_identifier_to_node(identifier, schema, info_provider, expect, &top_filter_for_pipeline())?)
            } {
                match this_top {
                    Node::Namespace(namespace) => {
                        current_space = Some(namespace);
                    }
                    Node::PipelineItemDeclaration(pipeline_item_declaration) => {
                        let argument_list = unit.expression_at(index + 1).map(|e| e.kind.as_argument_list()).flatten();
                        let arguments = fetch_argument_list_or_empty(argument_list, schema, info_provider, namespace, diagnostics)?;
                        if let Some(pipeline_item) = namespace.pipeline_item_at_path(&pipeline_item_declaration.str_path()) {
                            pipeline.items.push(BoundedItem {
                                path: pipeline_item.path().clone(),
                                arguments: arguments.clone(),
                                call: Arc::new(pipeline_item.creator().call(arguments)),
                                cast_output_type: pipeline_resolved.items_resolved.get(item_index).map(|r| r.output_type.clone()),
                            });
                        }
                        current_space = None;
                        item_index += 1;
                    }
                    _ => unreachable!()
                }
            } else {
                Err(Error::new("pipeline item not found"))?
            }
        }
    }
    Ok(Value::from(pipeline))
}