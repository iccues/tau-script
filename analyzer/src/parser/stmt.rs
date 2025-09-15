use crate::token::operator::Operator;
use crate::error::FrontendResult as Result;
use crate::lexer::token_peeker::TokenPeeker;

use crate::parser::expr::expr::Expr;

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
