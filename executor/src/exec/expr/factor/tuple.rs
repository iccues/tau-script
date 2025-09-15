use analyzer::parser::expr::factor::tuple::TupleExpr;
use object::core::prelude::*;
use object::ext::core_type::tuple::Tuple;
use crate::error::ExecutorResult;
use crate::exec::Exec;

impl Exec for TupleExpr {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        let mut args = Vec::new();
        for expr in self.exprs.iter() {
            args.push(expr.exec(env)?);
        }
        Ok(Tuple::new(args))
    }
}
