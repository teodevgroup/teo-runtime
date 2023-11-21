use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_arith_expr::fetch_arith_expr;
use crate::schema::fetch::fetchers::fetch_identifier::fetch_identifier;
use crate::schema::fetch::fetchers::fetch_literals::{fetch_array_literal, fetch_dictionary_literal, fetch_enum_variant_literal, fetch_tuple_literal};
use crate::schema::fetch::fetchers::fetch_pipeline::fetch_pipeline;
use crate::schema::fetch::fetchers::fetch_unit::fetch_unit;

pub fn fetch_expression_kind<I>(expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    match &expression.kind {
        ExpressionKind::Group(g) => fetch_expression(g.expression(), schema, info_provider, expect, namespace),
        ExpressionKind::ArithExpr(a) => fetch_arith_expr(a, schema, info_provider, expect, namespace),
        ExpressionKind::NumericLiteral(_) => unreachable!(),
        ExpressionKind::StringLiteral(_) => unreachable!(),
        ExpressionKind::RegexLiteral(_) => unreachable!(),
        ExpressionKind::BoolLiteral(_) => unreachable!(),
        ExpressionKind::NullLiteral(_) => unreachable!(),
        ExpressionKind::EnumVariantLiteral(e) => fetch_enum_variant_literal(e, schema, info_provider, expect, namespace),
        ExpressionKind::TupleLiteral(t) => fetch_tuple_literal(t, schema, info_provider, expect, namespace),
        ExpressionKind::ArrayLiteral(a) => fetch_array_literal(a, schema, info_provider, expect, namespace),
        ExpressionKind::DictionaryLiteral(d) => fetch_dictionary_literal(d, schema, info_provider, expect, namespace),
        ExpressionKind::Identifier(i) => fetch_identifier(i, schema, info_provider, expect, namespace),
        ExpressionKind::ArgumentList(_) => unreachable!(),
        ExpressionKind::Subscript(_) => unreachable!(),
        ExpressionKind::Unit(u) => fetch_unit(u, schema, info_provider, expect, namespace),
        ExpressionKind::Pipeline(p) => fetch_pipeline(p, schema, info_provider, expect, namespace),
        ExpressionKind::IntSubscript(_) => unreachable!(),
        ExpressionKind::NamedExpression(_) => unreachable!(),
        ExpressionKind::BracketExpression(e) => fetch_expression(e.expression(), schema, info_provider, expect, namespace),
    }
}