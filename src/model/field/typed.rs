use teo_parser::r#type::Type;
use crate::model::field::is_optional::IsOptional;

pub trait Typed: IsOptional {

    fn r#type(&self) -> &Type;
}