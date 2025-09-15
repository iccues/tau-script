use object::core::prelude::*;
use analyzer::parser::stmt::Stmt;
use crate::error::ExecutorResult;
use crate::exec::Exec;


impl Exec for Stmt {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        match self {
            Stmt::Expr(expr) => {
                expr.exec(env)
            }
            _ => unimplemented!(),
        }
    }
}
