use frontend_library::token::operator::Operator;
use lexer::token_peeker::TokenPeeker;

use crate::expr::expr::Expr;
use frontend_library::error::FrontendResult as Result;
use crate::expr::factor::tuple::TupleExpr;

#[derive(Debug)]
pub struct CallExpr {
    pub func: Box<Expr>,
    pub args: Box<Expr>,
}

impl CallExpr {
    pub fn try_parse(func: Box<Expr>, peeker: &mut TokenPeeker) -> Result<(bool, Box<Expr>)> {
        if peeker.peek()?.eq(&Operator::OpenParen) {
            let args = TupleExpr::parse(peeker)?;
            Ok((true, Box::new(Expr::Call(CallExpr { func, args }))))
        } else {
            Ok((false, func))
        }
    }
}
