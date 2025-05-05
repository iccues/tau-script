use lexer::{stream::peekable::cursor::Cursor, token::{operator::Operator, TokenBox}};

use crate::expr::expr::Expr;
use error::Result;
use crate::expr::factor::tuple::TupleExpr;

#[derive(Debug)]
pub struct CallExpr {
    pub func: Box<Expr>,
    pub args: Box<Expr>,
}

impl CallExpr {
    pub fn try_parse(func: Box<Expr>, cursor: &mut Cursor<TokenBox>) -> Result<(bool, Box<Expr>)> {
        if cursor.peek()?.eq(&Operator::OpenParen) {
            let args = TupleExpr::parse(cursor)?;
            Ok((true, Box::new(Expr::Call(CallExpr { func, args }))))
        } else {
            Ok((false, func))
        }
    }
}
