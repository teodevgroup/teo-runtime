use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use teo_parser::ast::arith_expr::{ArithExpr, ArithExprOperator};
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::value::range::Range;
use crate::value::Value;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_expression::fetch_expression;

pub fn fetch_arith_expr<I>(arith_expr: &ArithExpr, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Value> where I: InfoProvider {
    match arith_expr {
        ArithExpr::Expression(e) => fetch_expression(e.as_ref(), schema, info_provider, expect, namespace),
        ArithExpr::UnaryOperation(u) => {
            let rhs = fetch_arith_expr(u.rhs(), schema, info_provider, expect, namespace)?;
            match u.op {
                ArithExprOperator::Neg => Ok(Value::from(rhs.neg()?)),
                ArithExprOperator::BitNeg => Ok(Value::from(rhs.not()?)),
                ArithExprOperator::Not => Ok(Value::from(rhs.normal_not())),
                _ => unreachable!(),
            }
        }
        ArithExpr::BinaryOperation(b) => {
            let lhs = fetch_arith_expr(b.lhs(), schema, info_provider, expect, namespace)?;
            let rhs = fetch_arith_expr(b.rhs(), schema, info_provider, expect, namespace)?;
            match b.op {
                ArithExprOperator::Add => Ok(Value::from(lhs.add(&rhs)?)),
                ArithExprOperator::Sub => Ok(Value::from(lhs.sub(&rhs)?)),
                ArithExprOperator::Mul => Ok(Value::from(lhs.mul(&rhs)?)),
                ArithExprOperator::Div => Ok(Value::from(lhs.div(&rhs)?)),
                ArithExprOperator::Mod => Ok(Value::from(lhs.rem(&rhs)?)),
                ArithExprOperator::And => Ok(Value::from(lhs.and(&rhs))),
                ArithExprOperator::Or => Ok(Value::from(lhs.or(&rhs))),
                ArithExprOperator::BitAnd => Ok(Value::from(lhs.bitand(&rhs)?)),
                ArithExprOperator::BitXor => Ok(Value::from(lhs.bitxor(&rhs)?)),
                ArithExprOperator::BitOr => Ok(Value::from(lhs.bitor(&rhs)?)),
                ArithExprOperator::BitLS => Ok(Value::from(lhs.shl(&rhs)?)),
                ArithExprOperator::BitRS => Ok(Value::from(lhs.shr(&rhs)?)),
                ArithExprOperator::NullishCoalescing => Ok(if lhs.is_null() { rhs } else { lhs }),
                ArithExprOperator::Gt => Ok(Value::from(lhs.gt(&rhs))),
                ArithExprOperator::Gte => Ok(Value::from(lhs >= rhs)),
                ArithExprOperator::Lt => Ok(Value::from(lhs.lt(&rhs))),
                ArithExprOperator::Lte => Ok(Value::from(lhs <= rhs)),
                ArithExprOperator::Eq => Ok(Value::from(lhs.eq(&rhs))),
                ArithExprOperator::Neq => Ok(Value::from(!lhs.eq(&rhs))),
                ArithExprOperator::RangeOpen => Ok(Value::from(Value::Range(build_range(lhs, rhs, false)))),
                ArithExprOperator::RangeClose => Ok(Value::from(Value::Range(build_range(lhs, rhs, true)))),
                _ => unreachable!()
            }
        }
        ArithExpr::UnaryPostfixOperation(p) => {
            fetch_arith_expr(p.lhs(), schema, info_provider, expect, namespace)
        }
    }
}

fn build_range(lhs: Value, rhs: Value, closed: bool) -> Range {
    Range {
        closed,
        start: Box::new(lhs),
        end: Box::new(rhs),
    }
}