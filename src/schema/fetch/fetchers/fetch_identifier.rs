use teo_parser::ast::identifier::Identifier;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::object::Object;

pub fn fetch_identifier<I>(identifier: &Identifier, schema: &Schema, info_provider: &I, _expect: &Type) -> Result<Object> where I: InfoProvider {
    unreachable!()
}