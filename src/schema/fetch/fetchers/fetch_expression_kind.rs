use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::namespace;
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_arith_expr::fetch_arith_expr;
use crate::schema::fetch::fetchers::fetch_identifier::fetch_identifier;
use crate::schema::fetch::fetchers::fetch_literals::{fetch_array_literal, fetch_dictionary_literal, fetch_enum_variant_literal, fetch_tuple_literal};
use crate::schema::fetch::fetchers::fetch_pipeline::fetch_pipeline;
use crate::schema::fetch::fetchers::fetch_unit::fetch_unit;
use crate::value::Value;

pub fn fetch_expression_kind<I>(expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type, namespace: &namespace::Builder, diagnostics: &mut Diagnostics) -> Result<Value> where I: InfoProvider {
    match &expression.kind {
        ExpressionKind::Group(g) => fetch_expression(g.expression(), schema, info_provider, expect, namespace, diagnostics),
        ExpressionKind::ArithExpr(a) => fetch_arith_expr(a, schema, info_provider, expect, namespace, diagnostics),
        ExpressionKind::NumericLiteral(_) => unreachable!(),
        ExpressionKind::StringLiteral(_) => unreachable!(),
        ExpressionKind::RegexLiteral(_) => unreachable!(),
        ExpressionKind::BoolLiteral(_) => unreachable!(),
        ExpressionKind::NullLiteral(_) => unreachable!(),
        ExpressionKind::EnumVariantLiteral(e) => fetch_enum_variant_literal(e, schema, info_provider, &expect.expect_for_enum_variant_literal(), namespace, diagnostics),
        ExpressionKind::TupleLiteral(t) => fetch_tuple_literal(t, schema, info_provider, &expect.expect_for_tuple_literal(), namespace, diagnostics),
        ExpressionKind::ArrayLiteral(a) => fetch_array_literal(a, schema, info_provider, &expect.expect_for_array_literal(), namespace, diagnostics),
        ExpressionKind::DictionaryLiteral(d) => fetch_dictionary_literal(d, schema, info_provider, &expect.expect_for_dictionary_literal(), namespace, diagnostics),
        ExpressionKind::Identifier(i) => fetch_identifier(i, schema, info_provider, expect, namespace, diagnostics),
        ExpressionKind::ArgumentList(_) => unreachable!(),
        ExpressionKind::Subscript(_) => unreachable!(),
        ExpressionKind::Unit(u) => fetch_unit(u, schema, info_provider, expect, namespace, diagnostics),
        ExpressionKind::Pipeline(p) => fetch_pipeline(p, schema, info_provider, &expect.expect_for_pipeline(), namespace, diagnostics),
        ExpressionKind::IntSubscript(_) => unreachable!(),
        ExpressionKind::NamedExpression(_) => unreachable!(),
        ExpressionKind::BracketExpression(e) => fetch_expression(e.expression(), schema, info_provider, expect, namespace, diagnostics),
        ExpressionKind::EmptyPipeline(_) => unreachable!(),
        ExpressionKind::TypeAsValueExpression(t) => Ok(Value::Type(t.type_expr().resolved().clone())),
    }
}