use teo_parser::ast::argument_list::ArgumentList;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::namespace::Namespace;

pub fn fetch_argument_list<I>(argument_list: &ArgumentList, schema: &Schema, info_provider: &I, namespace: &Namespace) -> Result<Arguments> where I: InfoProvider {
    unreachable!()
}

pub fn fetch_argument_list_or_empty<I>(argument_list: Option<&ArgumentList>, schema: &Schema, info_provider: &I, namespace: &Namespace) -> Result<Arguments> where I: InfoProvider {
    if let Some(argument_list) = argument_list {
        fetch_argument_list(argument_list, schema, info_provider, namespace)
    } else {
        Ok(Arguments::default())
    }
}