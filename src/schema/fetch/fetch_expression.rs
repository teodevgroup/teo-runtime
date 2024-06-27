use teo_parser::ast::expression::Expression;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use crate::value::Value;
use teo_result::Result;
use crate::namespace;
use crate::namespace::Namespace;
use crate::schema::fetch::fetchers::fetch_expression_kind::fetch_expression_kind;

pub fn fetch_expression<I>(expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    if let Some(value) = expression.resolved().value() {
        // we separate enum variants and interface enum variants
        // so that resolved value in parser might be incorrect type
        // fetch expression value
        if let Some(enum_reference) = expression.resolved().r#type().as_enum_variant() {
            let enum_definition = schema.find_top_by_path(enum_reference.path()).unwrap().as_enum().unwrap();
            if enum_definition.interface {
                fetch_expression_kind(expression, schema, info_provider, &expect.expect_for_enum_variant_literal(), namespace, diagnostics)
            } else {
                Ok(Value::from(value.clone()))
            }
        } else if let Some(inner) = expression.resolved().r#type().as_array() {
            if inner.unwrap_optional().is_synthesized_interface_enum_reference() || inner.unwrap_optional().is_synthesized_interface_enum() {
                fetch_expression_kind(expression, schema, info_provider, &expect.expect_for_enum_variant_literal(), namespace, diagnostics)
            } else {
                Ok(Value::from(value.clone()))
            }
        } else if let Some(_) = expression.resolved().r#type().as_synthesized_interface_enum() {
            fetch_expression_kind(expression, schema, info_provider, &expect.expect_for_enum_variant_literal(), namespace, diagnostics)
        } else if let Some(_) = expression.resolved().r#type().as_synthesized_interface_enum_reference() {
            fetch_expression_kind(expression, schema, info_provider, &expect.expect_for_enum_variant_literal(), namespace, diagnostics)
        } else {
            Ok(Value::from(value.clone()))
        }
    } else {
        fetch_expression_kind(expression, schema, info_provider, expect, namespace, diagnostics)
    }
}

pub fn fetch_expression_or_default<I, T>(expression: Option<&Expression>, schema: &Schema, info_provider: &I, default: T, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider, T: Into<Value> {
    if let Some(expression) = expression {
        fetch_expression(expression, schema, info_provider, expect, namespace, diagnostics)
    } else {
        Ok(default.into())
    }
}

pub fn fetch_expression_or_null<I>(expression: Option<&Expression>, schema: &Schema, info_provider: &I, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    if let Some(expression) = expression {
        fetch_expression(expression, schema, info_provider, expect, namespace, diagnostics)
    } else {
        Ok(Value::from(Value::Null))
    }
}

pub fn fetch_dictionary_key_expression<I>(expression: &Expression, schema: &Schema, info_provider: &I, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    if let Some(value) = expression.resolved().value() {
        Ok(Value::from(value.clone()))
    } else {
        fetch_expression_kind(expression, schema, info_provider, &Type::String, namespace, diagnostics)
    }
}