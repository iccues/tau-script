use object::{object::object::Object, types::{control::undefined::Undefined, primitive::{bool::Bool, number::Integer, string::String_}}};
use parser::stmt::Stmt;
use token::operator::Operator;
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
                    Operator::NotEq => {
                        left.get_member("ne").call(Tuple::new(vec![right]))
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
            Expr::If(expr) => {
                let condition = expr.condition.exec(env);
                if condition.get_data_match::<Bool>().is_some_and(|b| b.value) {
                    expr.then_block.exec(env)
                } else {
                    expr.else_block.as_ref().map(|block| block.exec(env)).unwrap_or_else(|| Undefined::new())
                }
            }
            Expr::While(expr) => {
                let mut result = Undefined::new();
                while expr.condition.exec(env).get_data_match::<Bool>().is_some_and(|b| b.value) {
                    result = expr.then_block.exec(env);
                }
                result
            }
            Expr::Block(expr) => {
                for stmt in expr.statments.iter() {
                    stmt.exec(env);
                }
                expr.end_value.as_ref().map(|end_value| end_value.exec(env)).unwrap_or_else(|| Undefined::new())
            }
            expr => {
                println!("Expr not implemented: {:?}", expr);
                unimplemented!()
            },
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
