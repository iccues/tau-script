use crate::error::FrontendResult as Result;
use crate::token::{operator::Operator, keyword::Keyword};
use crate::lexer::token_peeker::TokenPeeker;

use crate::parser::expr::expr::Expr;

#[derive(Debug)]
pub struct WhileExpr {
    pub condition: Box<Expr>,
    pub then_block: Box<Expr>,
}

impl WhileExpr {
    pub fn parse(peeker: &mut TokenPeeker) -> Result<Box<Expr>> {
        peeker.eat_eq(&Keyword::While)?;
        peeker.eat_eq(&Operator::OpenParen)?;
        let condition = Expr::parse(peeker)?;
        peeker.eat_eq(&Operator::CloseParen)?;
        let then_block = Expr::parse(peeker)?;
        Ok(Box::new(Expr::While(WhileExpr { condition, then_block })))
    }
}
