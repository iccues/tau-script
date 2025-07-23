use object::core::prelude::*;
use object::ext::object_ext::ObjectExt;
use object::ext::tuple;
use object::types::{primitive::{bool::ObjBool, numbers::ObjI64, string::ObjString}, unit::undefined::Undefined};
use object::ext::tuple::Tuple;
use frontend::parser::stmt::Stmt;
use frontend::frontend_library::token::operator::Operator;
use frontend::parser::expr::expr::Expr;
use frontend::parser::expr::factor::literal::Literal;

use crate::error::ExecutorResult;

pub trait Exec {
    fn exec(&self, env: &Object) -> ExecutorResult<Object>;
}

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
                let left = expr.left.exec(env)?;
                let right = expr.right.exec(env)?;
                match &*expr.operator {
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
            Expr::Literal(literal) => {
                literal.exec(env)
            }
            Expr::Identifier(identifier) => {
                Ok(env.get_member(identifier)?)
            }
            Expr::Tuple(exprs) => {
                let mut args = Vec::new();
                for expr in exprs.exprs.iter() {
                    args.push(expr.exec(env)?);
                }
                Ok(Tuple::new(args))
            }
            Expr::If(expr) => {
                let condition = expr.condition.exec(env)?;
                if condition.match_downcast::<ObjBool>()?.value {
                    expr.then_block.exec(env)
                } else {
                    if let Some(else_block) = expr.else_block.as_ref() {
                        else_block.exec(env)
                    } else {
                        Ok(Undefined::new())
                    }
                }
            }
            Expr::While(expr) => {
                let mut result: Object = Undefined::new();
                while expr.condition.exec(env)?.match_downcast::<ObjBool>()?.value {
                    result = expr.then_block.exec(env)?;
                }
                Ok(result)
            }
            Expr::Block(expr) => {
                for stmt in expr.statments.iter() {
                    stmt.exec(env)?;
                }
                if let Some(end_value) = expr.end_value.as_ref() {
                    end_value.exec(env)
                } else {
                    Ok(Undefined::new())
                }
            }
            expr => {
                println!("Expr not implemented: {:?}", expr);
                unimplemented!()
            },
        }
    }
}

impl Exec for Literal {
    fn exec(&self, _env: &Object) -> ExecutorResult<Object> {
        match self {
            Literal::String(literal) => {
                Ok(ObjString::new(literal.clone()))
            }
            Literal::Integer(literal) => {
                Ok(ObjI64::new(literal.parse().unwrap()))
            }
            _ => unimplemented!(),
        }
    }
}

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
