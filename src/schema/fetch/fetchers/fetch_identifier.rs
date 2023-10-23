use std::sync::Arc;
use teo_parser::ast::identifier::Identifier;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::reference::ReferenceType;
use teo_parser::ast::schema::Schema;
use teo_parser::ast::top::Top;
use teo_parser::r#type::Type;
use teo_parser::search::search_identifier_path::search_identifier_path_in_source;
use teo_parser::utils::top_filter::top_filter_for_reference_type;
use teo_result::{Error, Result};
use teo_teon::Value;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::schema::fetch::fetch_expression::fetch_expression;

pub fn fetch_identifier<I>(identifier: &Identifier, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let path = fetch_identifier_path(identifier, schema, info_provider, expect, namespace, &top_filter_for_reference_type(ReferenceType::Default))?;
    let top = schema.find_top_by_path(&path).unwrap();
    match top {
        Top::Config(c) => Err(Error::new("cannot resolve")),
        Top::Constant(c) => fetch_expression(&c.expression, schema, info_provider, expect, namespace),
        Top::Enum(e) => Err(Error::new("cannot resolve")),
        Top::Model(m) => Ok(Object::from(Value::from(m.string_path.clone()))),
        Top::DataSet(d) => Ok(Object::from(Value::from(d.string_path.clone()))),
        Top::Interface(i) => Err(Error::new("cannot resolve")),
        Top::Namespace(n) => Err(Error::new("cannot resolve")),
        _ => unreachable!(),
    }
}

pub fn fetch_identifier_path<I>(identifier: &Identifier, schema: &Schema, info_provider: &I, _expect: &Type, namespace: &Namespace, filter: &Arc<dyn Fn(&Top) -> bool>) -> Result<Vec<usize>> where I: InfoProvider {
    Ok(search_identifier_path_in_source(
        schema,
        schema.source(info_provider.source_id()).unwrap(),
        &info_provider.namespace_str_path(),
        &vec![identifier.name()],
        filter,
        info_provider.availability()
    ).unwrap())
}