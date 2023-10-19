use teo_parser::ast::expression::Expression;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_teon::Value;
use crate::object::Object;

pub fn fetch_expression<I>(expression: &Expression, schema: &Schema, info_provider: I) -> Object where I: InfoProvider {
    unreachable!()
}

pub fn fetch_expression_or_default<I>(expression: Option<&Expression>, schema: &Schema, info_provider: I, default: Object) -> Object where I: InfoProvider {
    default
}

pub fn fetch_expression_or_null<I>(expression: Option<&Expression>, schema: &Schema, info_provider: I) -> Object where I: InfoProvider {
    Object::from(Value::Null)
}