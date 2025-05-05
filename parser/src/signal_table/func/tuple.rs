use error::Result;
use lexer::{stream::peekable::cursor::Cursor, token::{operator::Operator, TokenBox}};

use super::expr::Expr;

#[derive(Debug)]
pub struct TupleExpr {
    pub exprs: Vec<Box<Expr>>,
}

impl TupleExpr {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let mut exprs = Vec::new();
        cursor.eat_eq(&Operator::OpenParen)?;
        while cursor.eat_eq(&Operator::CloseParen).is_err() {
            exprs.push(Expr::parse(cursor)?);
            if cursor.eat_eq(&Operator::Comma).is_err() {
                cursor.eat_eq(&Operator::CloseParen)?;
                break;
            }
        }
        Ok(Box::new(Expr::Tuple(Self { exprs })))
    }

    pub fn parse_or_group(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let mut exprs = Vec::new();
        cursor.eat_eq(&Operator::OpenParen)?;
        exprs.push(Expr::parse(cursor)?);
        if cursor.eat_eq(&Operator::CloseParen).is_ok() {
            return Ok(exprs.pop().unwrap());
        }

        cursor.eat_eq(&Operator::Comma)?;
        while cursor.eat_eq(&Operator::CloseParen).is_err() {
            exprs.push(Expr::parse(cursor)?);
            if cursor.eat_eq(&Operator::Comma).is_err() {
                cursor.eat_eq(&Operator::CloseParen)?;
                break;
            }
        }
        Ok(Box::new(Expr::Tuple(Self { exprs })))
    }
}
