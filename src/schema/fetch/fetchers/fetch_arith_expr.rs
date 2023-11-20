use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use teo_parser::ast::arith_expr::{ArithExpr, ArithExprOperator};
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use teo_teon::types::range::Range;
use teo_teon::Value;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::schema::fetch::fetch_expression::fetch_expression;

pub fn fetch_arith_expr<I>(arith_expr: &ArithExpr, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    match arith_expr {
        ArithExpr::Expression(e) => fetch_expression(e.as_ref(), schema, info_provider, expect, namespace),
        ArithExpr::UnaryOperation(u) => {
            let rhs = fetch_arith_expr(u.rhs(), schema, info_provider, expect, namespace)?;
            match u.op {
                ArithExprOperator::Neg => Ok(Object::from(rhs.as_teon().unwrap().neg()?)),
                ArithExprOperator::BitNeg => Ok(Object::from(rhs.as_teon().unwrap().not()?)),
                ArithExprOperator::Not => Ok(Object::from(rhs.as_teon().unwrap().normal_not())),
                _ => unreachable!(),
            }
        }
        ArithExpr::BinaryOperation(b) => {
            let lhs = fetch_arith_expr(b.lhs(), schema, info_provider, expect, namespace)?;
            let rhs = fetch_arith_expr(b.rhs(), schema, info_provider, expect, namespace)?;
            match b.op {
                ArithExprOperator::Add => Ok(Object::from(lhs.as_teon().unwrap().add(rhs.as_teon().unwrap())?)),
                ArithExprOperator::Sub => Ok(Object::from(lhs.as_teon().unwrap().sub(rhs.as_teon().unwrap())?)),
                ArithExprOperator::Mul => Ok(Object::from(lhs.as_teon().unwrap().mul(rhs.as_teon().unwrap())?)),
                ArithExprOperator::Div => Ok(Object::from(lhs.as_teon().unwrap().div(rhs.as_teon().unwrap())?)),
                ArithExprOperator::Mod => Ok(Object::from(lhs.as_teon().unwrap().rem(rhs.as_teon().unwrap())?)),
                ArithExprOperator::And => Ok(Object::from(lhs.as_teon().unwrap().and(rhs.as_teon().unwrap()))),
                ArithExprOperator::Or => Ok(Object::from(lhs.as_teon().unwrap().or(rhs.as_teon().unwrap()))),
                ArithExprOperator::BitAnd => Ok(Object::from(lhs.as_teon().unwrap().bitand(rhs.as_teon().unwrap())?)),
                ArithExprOperator::BitXor => Ok(Object::from(lhs.as_teon().unwrap().bitxor(rhs.as_teon().unwrap())?)),
                ArithExprOperator::BitOr => Ok(Object::from(lhs.as_teon().unwrap().bitor(rhs.as_teon().unwrap())?)),
                ArithExprOperator::BitLS => Ok(Object::from(lhs.as_teon().unwrap().shl(rhs.as_teon().unwrap())?)),
                ArithExprOperator::BitRS => Ok(Object::from(lhs.as_teon().unwrap().shr(rhs.as_teon().unwrap())?)),
                ArithExprOperator::NullishCoalescing => Ok(if lhs.as_teon().unwrap().is_null() { rhs } else { lhs }),
                ArithExprOperator::Gt => Ok(Object::from(lhs.as_teon().unwrap().gt(rhs.as_teon().unwrap()))),
                ArithExprOperator::Gte => Ok(Object::from(lhs.as_teon().unwrap() >= rhs.as_teon().unwrap())),
                ArithExprOperator::Lt => Ok(Object::from(lhs.as_teon().unwrap().lt(rhs.as_teon().unwrap()))),
                ArithExprOperator::Lte => Ok(Object::from(lhs.as_teon().unwrap() <= rhs.as_teon().unwrap())),
                ArithExprOperator::Eq => Ok(Object::from(lhs.as_teon().unwrap().eq(rhs.as_teon().unwrap()))),
                ArithExprOperator::Neq => Ok(Object::from(!lhs.as_teon().unwrap().eq(rhs.as_teon().unwrap()))),
                ArithExprOperator::RangeOpen => Ok(Object::from(Value::Range(build_range(lhs, rhs, false)))),
                ArithExprOperator::RangeClose => Ok(Object::from(Value::Range(build_range(lhs, rhs, true)))),
                _ => unreachable!()
            }
        }
        ArithExpr::UnaryPostfixOperation(p) => {
            fetch_arith_expr(p.lhs(), schema, info_provider, expect, namespace)
        }
    }
}

fn build_range(lhs: Object, rhs: Object, closed: bool) -> Range {
    Range {
        closed,
        start: Box::new(lhs.as_teon().unwrap().clone()),
        end: Box::new(rhs.as_teon().unwrap().clone()),
    }
}