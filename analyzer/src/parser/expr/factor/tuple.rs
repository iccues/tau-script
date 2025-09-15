use crate::error::FrontendResult as Result;
use crate::token::operator::Operator;
use crate::lexer::token_peeker::TokenPeeker;

use crate::parser::expr::expr::Expr;

#[derive(Debug)]
pub struct TupleExpr {
    pub exprs: Vec<Box<Expr>>,
}

impl TupleExpr {
    pub fn parse(peeker: &mut TokenPeeker) -> Result<Box<Expr>> {
        let mut exprs = Vec::new();
        peeker.eat_eq(&Operator::OpenParen)?;
        while peeker.eat_eq(&Operator::CloseParen).is_err() {
            exprs.push(Expr::parse(peeker)?);
            if peeker.eat_eq(&Operator::Comma).is_err() {
                peeker.eat_eq(&Operator::CloseParen)?;
                break;
            }
        }
        Ok(Box::new(Expr::Tuple(TupleExpr { exprs })))
    }

    pub fn parse_or_group(peeker: &mut TokenPeeker) -> Result<Box<Expr>> {
        let mut exprs = Vec::new();
        peeker.eat_eq(&Operator::OpenParen)?;
        exprs.push(Expr::parse(peeker)?);
        if peeker.eat_eq(&Operator::CloseParen).is_ok() {
            return Ok(exprs.pop().unwrap());
        }

        peeker.eat_eq(&Operator::Comma)?;
        while peeker.eat_eq(&Operator::CloseParen).is_err() {
            exprs.push(Expr::parse(peeker)?);
            if peeker.eat_eq(&Operator::Comma).is_err() {
                peeker.eat_eq(&Operator::CloseParen)?;
                break;
            }
        }
        Ok(Box::new(Expr::Tuple(TupleExpr { exprs })))
    }
}
