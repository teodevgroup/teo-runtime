use teo_parser::ast::expression::Expression;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_teon::Value;
use teo_result::Result;
use crate::object::Object;
use crate::schema::fetch::fetchers::fetch_expression_kind::fetch_expression_kind;

pub fn fetch_expression<I>(expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type) -> Result<Object> where I: InfoProvider {
    if let Some(value) = expression.resolved().value() {
        Ok(Object::from(value.clone()))
    } else {
        fetch_expression_kind(expression, schema, info_provider, expect)
    }
}

pub fn fetch_expression_or_default<I, T>(expression: Option<&Expression>, schema: &Schema, info_provider: &I, default: T, expect: &Type) -> Result<Object> where I: InfoProvider, T: Into<Object> {
    if let Some(expression) = expression {
        fetch_expression(expression, schema, info_provider, expect)
    } else {
        Ok(default.into())
    }
}

pub fn fetch_expression_or_null<I>(expression: Option<&Expression>, schema: &Schema, info_provider: &I, expect: &Type) -> Result<Object> where I: InfoProvider {
    if let Some(expression) = expression {
        fetch_expression(expression, schema, info_provider, expect)
    } else {
        Ok(Object::from(Value::Null))
    }
}