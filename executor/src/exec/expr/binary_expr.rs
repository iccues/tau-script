use analyzer::parser::expr::binary_expr::BinaryExpr;
use analyzer::token::operator::Operator;
use object::core::prelude::*;
use object::ext::tuple;

use crate::error::ExecutorResult;
use crate::exec::Exec;

impl Exec for BinaryExpr {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        let left = self.left.exec(env)?;
        let right = self.right.exec(env)?;
        match &*self.operator {
            Operator::Plus => {
                Ok(left.get_member("add")?.call(tuple!(right)))
            }
            Operator::Eq => {
                Ok(left.get_member("set")?.call(tuple!(right)))
            }
            Operator::NotEq => {
                Ok(left.get_member("ne")?.call(tuple!(right)))
            }
            Operator::DoubleEq => {
                Ok(left.get_member("eq")?.call(tuple!(right)))
            }
            _ => unimplemented!(),
        }
    }
}
