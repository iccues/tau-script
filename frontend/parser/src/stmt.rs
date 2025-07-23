use frontend_library::stream::peeker::Peeker;
use frontend_library::token::{operator::Operator, TokenBox};
use frontend_library::error::FrontendResult as Result;

use crate::expr::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
    Def,
}

impl Stmt {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Stmt> {
        let expr = Expr::parse(peeker)?;
        peeker.eat_eq(&Operator::Semi)?;
        Ok(Stmt::Expr(expr))
    }
}
