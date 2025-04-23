use parser::signal_table::func::{expr::Expr, literal::Literal};
use lexer::token::operator::Operator;

use crate::object::{number::Integer, string::String_, tuple::Tuple, ObjectBox};

pub trait ExprEval {
    fn eval(&self, env: &ObjectBox) -> ObjectBox;
}

impl ExprEval for Expr {
    fn eval(&self, env: &ObjectBox) -> ObjectBox {
        match self {
            Expr::BinaryExpr {
                left,
                operator, right
            } => {
                let left = left.eval(env);
                let right = right.eval(env);
                match &**operator {
                    Operator::Plus => {
                        left.get_member("add").call(Box::new(Tuple::new(vec![right])))
                    }
                    _ => unimplemented!(),
                }
            }
            Expr::Literal(literal) => {
                literal.eval(env)
            }
            Expr::Identifier(identifier) => {
                env.get_member(identifier)
            }
            _ => unimplemented!(),
        }
    }
}

impl ExprEval for Literal {
    fn eval(&self, _env: &ObjectBox) -> ObjectBox {
        match self {
            Literal::String(literal) => {
                Box::new(String_::new(literal))
            }
            Literal::Integer(literal) => {
                Box::new(Integer::new(literal))
            }
            _ => unimplemented!(),
        }
    }
}