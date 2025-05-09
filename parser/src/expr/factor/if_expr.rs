use error::Result;
use stream::peeker::Peeker;
use token::{operator::Operator, keyword::Keyword, TokenBox};

use crate::expr::expr::Expr;

#[derive(Debug)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_block: Box<Expr>,
    pub else_block: Option<Box<Expr>>,
}

impl IfExpr {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
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
