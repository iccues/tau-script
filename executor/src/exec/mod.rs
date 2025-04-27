use parser::signal_table::func::{expr::Expr, literal::Literal, stmt::Stmt};
use lexer::token::operator::Operator;

use interpreter::object::{number::Integer, string::String_, tuple::Tuple, Object};

pub trait Exec {
    fn exec(&self, env: &Object) -> Object;
}

impl Exec for Expr {
    fn exec(&self, env: &Object) -> Object {
        match self {
            Expr::BinaryExpr {
                left,
                operator, right
            } => {
                let left = left.exec(env);
                let right = right.exec(env);
                match &**operator {
                    Operator::Plus => {
                        left.get_member("add").call(Tuple::new(vec![right]))
                    }
                    Operator::Eq => {
                        left.get_member("set").call(Tuple::new(vec![right]))
                    }
                    _ => unimplemented!(),
                }
            }
            Expr::Literal(literal) => {
                literal.exec(env)
            }
            Expr::Identifier(identifier) => {
                env.get_member(identifier)
            }
            _ => unimplemented!(),
        }
    }
}

impl Exec for Literal {
    fn exec(&self, _env: &Object) -> Object {
        match self {
            Literal::String(literal) => {
                String_::new(literal)
            }
            Literal::Integer(literal) => {
                Integer::new(literal)
            }
            _ => unimplemented!(),
        }
    }
}

impl Exec for Stmt {
    fn exec(&self, env: &Object) -> Object {
        match self {
            Stmt::Expr(expr) => {
                expr.exec(env)
            }
            _ => unimplemented!(),
        }
    }
}
