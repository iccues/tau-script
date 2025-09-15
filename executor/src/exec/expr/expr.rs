use object::core::prelude::*;
use analyzer::parser::expr::expr::Expr;
use crate::error::ExecutorResult;
use crate::exec::Exec;


impl Exec for Expr {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        match self {
            Expr::Call(expr) => {
                let func = expr.func.exec(env)?;
                let args = expr.args.exec(env)?;
                Ok(func.try_call()?(args))
            }
            Expr::Dot(expr) => {
                let left = expr.left.exec(env)?;
                Ok(left.get_member(&*expr.right)?)
            }
            Expr::Binary(expr) => {
                expr.exec(env)
            }
            Expr::Literal(literal) => {
                literal.exec(env)
            }
            Expr::Tuple(expr) => {
                expr.exec(env)
            }
            Expr::If(expr) => {
                expr.exec(env)
            }
            Expr::While(expr) => {
                expr.exec(env)
            }
            Expr::Block(expr) => {
                expr.exec(env)
            }

            Expr::Identifier(identifier) => {
                Ok(env.get_member(identifier)?)
            }
            expr => {
                println!("Expr not implemented: {:?}", expr);
                unimplemented!()
            },
        }
    }
}
