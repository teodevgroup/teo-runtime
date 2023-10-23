use teo_parser::ast::unit::Unit;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::object::Object;

pub fn fetch_unit<I>(unit: &Unit, schema: &Schema, info_provider: &I, _expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    unreachable!()
}