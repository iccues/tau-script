use frontend_library::token::operator::Operator;
use frontend_library::error::FrontendResult as Result;
use lexer::token_peeker::TokenPeeker;

use crate::expr::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
    Def,
}

impl Stmt {
    pub fn parse(peeker: &mut TokenPeeker) -> Result<Stmt> {
        let expr = Expr::parse(peeker)?;
        peeker.eat_eq(&Operator::Semi)?;
        Ok(Stmt::Expr(expr))
    }
}
