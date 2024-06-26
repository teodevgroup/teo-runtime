use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::info_provider::InfoProvider;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::namespace;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list_or_empty;

pub fn fetch_decorator_arguments<I>(decorator: &teo_parser::ast::decorator::Decorator, schema: &Schema, info_provider: &I, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Arguments> where I: InfoProvider {
    fetch_argument_list_or_empty(decorator.argument_list(), schema, info_provider, namespace, diagnostics)
}