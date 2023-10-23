use teo_parser::ast::argument_list::ArgumentList;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::namespace::Namespace;

pub fn fetch_argument_list<I>(argument_list: &ArgumentList, schema: &Schema, info_provider: &I, namespace: &Namespace) -> Result<Arguments> where I: InfoProvider {
    unreachable!()
}