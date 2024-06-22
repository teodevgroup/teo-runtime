use maplit::btreemap;
use teo_parser::ast::argument_list::ArgumentList;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::namespace::builder::NamespaceBuilder;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_expression::fetch_expression;

pub fn fetch_argument_list<I>(argument_list: &ArgumentList, schema: &Schema, info_provider: &I, namespace: &NamespaceBuilder, diagnostics: &mut Diagnostics) -> Result<Arguments> where I: InfoProvider {
    let mut map = btreemap! {};
    for argument in argument_list.arguments() {
        map.insert(
            argument.resolved_name().unwrap().to_owned(),
            fetch_expression(argument.value(), schema, info_provider, &argument.resolved().expect, namespace, diagnostics)?,
        );
    }
    Ok(Arguments::new(map))
}

pub fn fetch_argument_list_or_empty<I>(argument_list: Option<&ArgumentList>, schema: &Schema, info_provider: &I, namespace: &NamespaceBuilder, diagnostics: &mut Diagnostics) -> Result<Arguments> where I: InfoProvider {
    if let Some(argument_list) = argument_list {
        fetch_argument_list(argument_list, schema, info_provider, namespace, diagnostics)
    } else {
        Ok(Arguments::default())
    }
}