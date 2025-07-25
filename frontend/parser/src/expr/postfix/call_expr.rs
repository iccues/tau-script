use frontend_library::stream::peeker::Peeker;
use frontend_library::token::{operator::Operator, TokenBox};

use crate::expr::expr::Expr;
use frontend_library::error::FrontendResult as Result;
use crate::expr::factor::tuple::TupleExpr;

#[derive(Debug)]
pub struct CallExpr {
    pub func: Box<Expr>,
    pub args: Box<Expr>,
}

impl CallExpr {
    pub fn try_parse(func: Box<Expr>, peeker: &mut Peeker<TokenBox>) -> Result<(bool, Box<Expr>)> {
        if peeker.peek()?.eq(&Operator::OpenParen) {
            let args = TupleExpr::parse(peeker)?;
            Ok((true, Box::new(Expr::Call(CallExpr { func, args }))))
        } else {
            Ok((false, func))
        }
    }
}
