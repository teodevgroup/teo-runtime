use teo_parser::ast::pipeline::Pipeline;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::object::Object;

pub fn fetch_pipeline<I>(pipeline: &Pipeline, schema: &Schema, info_provider: I, _expect: &Type) -> Result<Object> where I: InfoProvider {
    unreachable!()
}