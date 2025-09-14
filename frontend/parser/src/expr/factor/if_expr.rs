use frontend_library::error::FrontendResult as Result;
use frontend_library::token::{operator::Operator, keyword::Keyword};
use lexer::token_peeker::TokenPeeker;

use crate::expr::expr::Expr;

#[derive(Debug)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_block: Box<Expr>,
    pub else_block: Option<Box<Expr>>,
}

impl IfExpr {
    pub fn parse(peeker: &mut TokenPeeker) -> Result<Box<Expr>> {
        peeker.eat_eq(&Keyword::If)?;
        peeker.eat_eq(&Operator::OpenParen)?;
        let condition = Expr::parse(peeker)?;
        peeker.eat_eq(&Operator::CloseParen)?;
        let then_block = Expr::parse(peeker)?;
        let else_block = if peeker.eat_eq(&Keyword::Else).is_ok() {
            Some(Expr::parse(peeker)?)
        } else {
            None
        };
        Ok(Box::new(Expr::If(
            IfExpr {
                condition, then_block, else_block
            }
        )))
    }
}
