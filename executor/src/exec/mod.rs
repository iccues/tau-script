use object::{object::object::Object, types::primitive::{number::Integer, string::String_}};
use parser::stmt::Stmt;
use lexer::token::operator::Operator;
use object::types::compound::tuple::Tuple;
use parser::expr::expr::Expr;
use parser::expr::factor::literal::Literal;

pub trait Exec {
    fn exec(&self, env: &Object) -> Object;
}

impl Exec for Expr {
    fn exec(&self, env: &Object) -> Object {
        match self {
            Expr::Call(expr) => {
                let func = expr.func.exec(env);
                let args = expr.args.exec(env);
                func.call(args)
            }
            Expr::Dot(expr) => {
                let left = expr.left.exec(env);
                left.get_member(&*expr.right)
            }
            Expr::Binary(expr) => {
                let left = expr.left.exec(env);
                let right = expr.right.exec(env);
                match &*expr.operator {
                    Operator::Plus => {
                        left.get_member("add").call(Tuple::new(vec![right]))
                    }
                    Operator::Eq => {
                        left.get_member("set").call(Tuple::new(vec![right]))
                    }
                    Operator::DoubleEq => {
                        left.get_member("eq").call(Tuple::new(vec![right]))
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
            Expr::Tuple(exprs) => {
                let mut args = Vec::new();
                for expr in exprs.exprs.iter() {
                    args.push(expr.exec(env));
                }
                Tuple::new(args)
            }
            _ => unimplemented!(),
        }
    }
}

impl Exec for Literal {
    fn exec(&self, _env: &Object) -> Object {
        match self {
            Literal::String(literal) => {
                String_::new(literal.clone())
            }
            Literal::Integer(literal) => {
                Integer::new(literal.parse().unwrap())
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
