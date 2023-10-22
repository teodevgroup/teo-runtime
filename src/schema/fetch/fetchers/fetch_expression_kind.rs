use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::object::Object;
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_arith_expr::fetch_arith_expr;
use crate::schema::fetch::fetchers::fetch_identifier::fetch_identifier;
use crate::schema::fetch::fetchers::fetch_literals::{fetch_array_literal, fetch_dictionary_literal, fetch_enum_variant_literal, fetch_tuple_literal};
use crate::schema::fetch::fetchers::fetch_pipeline::fetch_pipeline;
use crate::schema::fetch::fetchers::fetch_unit::fetch_unit;

pub fn fetch_expression_kind<I>(expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type) -> Result<Object> where I: InfoProvider {
    match &expression.kind {
        ExpressionKind::Group(g) => fetch_expression(&g.expression.as_ref(), schema, info_provider, expect),
        ExpressionKind::ArithExpr(a) => fetch_arith_expr(a, schema, info_provider),
        ExpressionKind::NumericLiteral(n) => unreachable!(),
        ExpressionKind::StringLiteral(s) => unreachable!(),
        ExpressionKind::RegexLiteral(r) => unreachable!(),
        ExpressionKind::BoolLiteral(b) => unreachable!(),
        ExpressionKind::NullLiteral(n) => unreachable!(),
        ExpressionKind::EnumVariantLiteral(e) => fetch_enum_variant_literal(e, schema, info_provider, expect),
        ExpressionKind::TupleLiteral(t) => fetch_tuple_literal(t, schema, info_provider, expect),
        ExpressionKind::ArrayLiteral(a) => fetch_array_literal(a, schema, info_provider, expect),
        ExpressionKind::DictionaryLiteral(d) => fetch_dictionary_literal(d, schema, info_provider, expect),
        ExpressionKind::Identifier(i) => fetch_identifier(i, schema, info_provider, expect),
        ExpressionKind::ArgumentList(_) => unreachable!(),
        ExpressionKind::Subscript(_) => unreachable!(),
        ExpressionKind::Call(_) => unreachable!(),
        ExpressionKind::Unit(u) => fetch_unit(u, schema, info_provider, expect),
        ExpressionKind::Pipeline(p) => fetch_pipeline(p, schema, info_provider, expect),
    }
}