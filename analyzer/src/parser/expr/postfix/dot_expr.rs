use crate::token::{identifier::Identifier, operator::Operator};
use crate::lexer::token_peeker::TokenPeeker;

use crate::parser::expr::expr::Expr;
use crate::error::FrontendResult as Result;

#[derive(Debug)]
pub struct DotExpr {
    pub left: Box<Expr>,
    pub right: String,
}

impl DotExpr {
    pub fn try_parse(left: Box<Expr>, peeker: &mut TokenPeeker) -> Result<(bool, Box<Expr>)> {
        if peeker.eat_eq(&Operator::Dot).is_ok() {
            let right = peeker.eat_type::<Identifier>()?.name();
            Ok((true, Box::new(Expr::Dot(DotExpr { left, right }))))
        } else {
            Ok((false, left))
        }
    }
}
