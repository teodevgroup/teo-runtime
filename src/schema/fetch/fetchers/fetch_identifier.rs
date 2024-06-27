use std::sync::Arc;
use teo_parser::ast::identifier::Identifier;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::ast::node::Node;
use teo_parser::ast::reference_space::ReferenceSpace;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::expr::ExprInfo;
use teo_parser::r#type::Type;
use teo_parser::search::search_identifier_path::{search_identifier_path_names_with_filter_to_expr_info, search_identifier_path_names_with_filter_to_top};
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::utils::top_filter::top_filter_for_reference_type;
use teo_result::{Error, Result};
use crate::namespace;
use crate::value::Value;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_expression::fetch_expression;

pub fn fetch_identifier<I>(identifier: &Identifier, schema: &Schema, info_provider: &I, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    let top = fetch_identifier_to_node(identifier, schema, info_provider, expect,  &top_filter_for_reference_type(ReferenceSpace::Default))?;
    match top {
        Node::Config(c) => Err(Error::new("cannot resolve")),
        Node::ConstantDeclaration(c) => fetch_expression(c.expression(), schema, info_provider, expect, namespace, diagnostics),
        Node::Enum(e) => Err(Error::new("cannot resolve")),
        Node::Model(m) => Ok(Value::from(m.string_path().clone())),
        Node::DataSet(d) => Ok(Value::from(d.string_path().clone())),
        Node::InterfaceDeclaration(i) => Err(Error::new("cannot resolve")),
        Node::Namespace(n) => Err(Error::new("cannot resolve")),
        _ => unreachable!(),
    }
}

pub fn fetch_identifier_to_node<'a, I>(identifier: &Identifier, schema: &'a Schema, info_provider: &I, _expect: &Type, filter: &Arc<dyn Fn(&Node) -> bool>) -> Result<&'a Node> where I: InfoProvider {
    Ok(search_identifier_path_names_with_filter_to_top(
        &vec![identifier.name()],
        schema,
        schema.source(info_provider.source_id()).unwrap(),
        &info_provider.namespace_str_path(),
        filter,
        info_provider.availability(),
    ).unwrap())
}

pub fn fetch_identifier_to_expr_info<'a, I>(identifier: &Identifier, schema: &'a Schema, info_provider: &I, _expect: &Type, filter: &Arc<dyn Fn(&Node) -> bool>) -> Result<ExprInfo> where I: InfoProvider {
    Ok(search_identifier_path_names_with_filter_to_expr_info(
        &vec![identifier.name()],
        schema,
        schema.source(info_provider.source_id()).unwrap(),
        &info_provider.namespace_str_path(),
        filter,
        info_provider.availability(),
    ).unwrap())
}