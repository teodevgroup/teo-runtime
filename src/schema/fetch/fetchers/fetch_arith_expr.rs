use teo_parser::ast::arith::ArithExpr;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_result::Result;
use crate::namespace::Namespace;
use crate::object::Object;

pub fn fetch_arith_expr<I>(arith_expr: &ArithExpr, schema: &Schema, info_provider: &I, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    unreachable!()
}