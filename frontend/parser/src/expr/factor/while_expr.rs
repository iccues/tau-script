use frontend_library::error::FrontendResult as Result;
use frontend_library::stream::peeker::Peeker;
use frontend_library::token::{operator::Operator, keyword::Keyword, TokenBox};

use crate::expr::expr::Expr;

#[derive(Debug)]
pub struct WhileExpr {
    pub condition: Box<Expr>,
    pub then_block: Box<Expr>,
}

impl WhileExpr {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        peeker.eat_eq(&Keyword::While)?;
        peeker.eat_eq(&Operator::OpenParen)?;
        let condition = Expr::parse(peeker)?;
        peeker.eat_eq(&Operator::CloseParen)?;
        let then_block = Expr::parse(peeker)?;
        Ok(Box::new(Expr::While(WhileExpr { condition, then_block })))
    }
}
