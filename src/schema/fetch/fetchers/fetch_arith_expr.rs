use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use teo_parser::ast::arith::{ArithExpr, Op};
use teo_parser::ast::info_provider::InfoProvider;
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
        ArithExpr::UnaryOp(u) => {
            let rhs = fetch_arith_expr(u.rhs.as_ref(), schema, info_provider, expect, namespace)?;
            match u.op {
                Op::Neg => Ok(Object::from(rhs.as_teon().unwrap().neg()?)),
                Op::BitNeg => Ok(Object::from(rhs.as_teon().unwrap().not()?)),
                Op::Not => Ok(Object::from(rhs.as_teon().unwrap().normal_not())),
                _ => unreachable!(),
            }
        }
        ArithExpr::BinaryOp(b) => {
            let lhs = fetch_arith_expr(b.lhs.as_ref(), schema, info_provider, expect, namespace)?;
            let rhs = fetch_arith_expr(b.rhs.as_ref(), schema, info_provider, expect, namespace)?;
            match b.op {
                Op::Add => Ok(Object::from(lhs.as_teon().unwrap().add(rhs.as_teon().unwrap())?)),
                Op::Sub => Ok(Object::from(lhs.as_teon().unwrap().sub(rhs.as_teon().unwrap())?)),
                Op::Mul => Ok(Object::from(lhs.as_teon().unwrap().mul(rhs.as_teon().unwrap())?)),
                Op::Div => Ok(Object::from(lhs.as_teon().unwrap().div(rhs.as_teon().unwrap())?)),
                Op::Mod => Ok(Object::from(lhs.as_teon().unwrap().rem(rhs.as_teon().unwrap())?)),
                Op::And => Ok(Object::from(lhs.as_teon().unwrap().and(rhs.as_teon().unwrap()))),
                Op::Or => Ok(Object::from(lhs.as_teon().unwrap().or(rhs.as_teon().unwrap()))),
                Op::BitAnd => Ok(Object::from(lhs.as_teon().unwrap().bitand(rhs.as_teon().unwrap())?)),
                Op::BitXor => Ok(Object::from(lhs.as_teon().unwrap().bitxor(rhs.as_teon().unwrap())?)),
                Op::BitOr => Ok(Object::from(lhs.as_teon().unwrap().bitor(rhs.as_teon().unwrap())?)),
                Op::BitLS => Ok(Object::from(lhs.as_teon().unwrap().shl(rhs.as_teon().unwrap())?)),
                Op::BitRS => Ok(Object::from(lhs.as_teon().unwrap().shr(rhs.as_teon().unwrap())?)),
                Op::NullishCoalescing => Ok(if lhs.as_teon().unwrap().is_null() { rhs } else { lhs }),
                Op::Gt => Ok(Object::from(lhs.as_teon().unwrap().gt(rhs.as_teon().unwrap()))),
                Op::Gte => Ok(Object::from(lhs.as_teon().unwrap() >= rhs.as_teon().unwrap())),
                Op::Lt => Ok(Object::from(lhs.as_teon().unwrap().lt(rhs.as_teon().unwrap()))),
                Op::Lte => Ok(Object::from(lhs.as_teon().unwrap() <= rhs.as_teon().unwrap())),
                Op::Eq => Ok(Object::from(lhs.as_teon().unwrap().eq(rhs.as_teon().unwrap()))),
                Op::Neq => Ok(Object::from(!lhs.as_teon().unwrap().eq(rhs.as_teon().unwrap()))),
                Op::RangeOpen => Ok(Object::from(Value::Range(build_range(lhs, rhs, false)))),
                Op::RangeClose => Ok(Object::from(Value::Range(build_range(lhs, rhs, true)))),
                _ => unreachable!()
            }
        }
        ArithExpr::UnaryPostfixOp(p) => {
            fetch_arith_expr(p.lhs.as_ref(), schema, info_provider, expect, namespace)
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