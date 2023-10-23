use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list_or_empty;

pub fn fetch_decorator_arguments<I>(decorator: &teo_parser::ast::decorator::Decorator, schema: &Schema, info_provider: &I, namespace: &Namespace) -> Result<Arguments> where I: InfoProvider {
    fetch_argument_list_or_empty(decorator.argument_list.as_ref(), schema, info_provider, namespace)
}