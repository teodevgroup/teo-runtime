use indexmap::indexmap;
use teo_parser::ast::literals::{TupleLiteral, ArrayLiteral, DictionaryLiteral, EnumVariantLiteral};
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use teo_teon::Value;
use crate::object::Object;
use crate::schema::fetch::fetch_expression::fetch_expression;

pub fn fetch_tuple_literal<I>(tuple_literal: &TupleLiteral, schema: &Schema, info_provider: I, _expect: &Type) -> Result<Object> where I: InfoProvider {
    let mut result = vec![];
    for expression in &tuple_literal.expressions {
        result.push(fetch_expression(expression, schema, &info_provider)?.as_teon().unwrap().clone());
    }
    Ok(Object::from(Value::Tuple(result)))
}

pub fn fetch_array_literal<I>(array_literal: &ArrayLiteral, schema: &Schema, info_provider: I, _expect: &Type) -> Result<Object> where I: InfoProvider {
    let mut result = vec![];
    for expression in &array_literal.expressions {
        result.push(fetch_expression(expression, schema, &info_provider)?.as_teon().unwrap().clone());
    }
    Ok(Object::from(Value::Array(result)))
}

pub fn fetch_dictionary_literal<I>(dictionary_literal: &DictionaryLiteral, schema: &Schema, info_provider: I, _expect: &Type) -> Result<Object> where I: InfoProvider {
    let mut result = indexmap!{};
    for (k_expression, v_expression) in &dictionary_literal.expressions {
        let k = fetch_expression(k_expression, schema, &info_provider)?.as_teon().unwrap().as_str().unwrap().to_owned();
        let v = fetch_expression(v_expression, schema, &info_provider)?.as_teon().unwrap().clone();
        result.insert(k, v);
    }
    Ok(Object::from(Value::Dictionary(result)))
}

pub fn fetch_enum_variant_literal<I>(enum_variant_literal: &EnumVariantLiteral, schema: &Schema, info_provider: I, expect: &Type) -> Result<Object> where I: InfoProvider {
    //enum_variant_literal.identifier
    unreachable!()
}