use lexer::{stream::peeker::Peeker, token::{identifier::Identifier, operator::Operator, TokenBox}};

use crate::expr::expr::Expr;
use error::Result;

#[derive(Debug)]
pub struct DotExpr {
    pub left: Box<Expr>,
    pub right: String,
}

impl DotExpr {
    pub fn try_parse(left: Box<Expr>, peeker: &mut Peeker<TokenBox>) -> Result<(bool, Box<Expr>)> {
        if peeker.eat_eq(&Operator::Dot).is_ok() {
            let right = peeker.eat_type::<Identifier>()?.name();
            Ok((true, Box::new(Expr::Dot(DotExpr { left, right }))))
        } else {
            Ok((false, left))
        }
    }
}
