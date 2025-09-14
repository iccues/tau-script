use frontend_library::token::keyword::Keyword;
use frontend_library::token::{
    number::{Float, Integer},
    string::StringToken,
};

use frontend_library::error::{FrontendError, FrontendResult as Result};
use lexer::token_peeker::TokenPeeker;

use crate::expr::expr::Expr;

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Literal {
    pub fn parse(peeker: &mut TokenPeeker) -> Result<Box<Expr>> {
        if let Ok(integer) = peeker.eat_type::<Integer>() {
            let value = integer.number();
            return Self::wrap_expr(Literal::Integer(value));
        }
        if let Ok(float) = peeker.eat_type::<Float>() {
            let value = float.number();
            return Self::wrap_expr(Literal::Float(value));
        }
        if let Ok(string) = peeker.eat_type::<StringToken>() {
            let value = string.string();
            return Self::wrap_expr(Literal::String(value));
        }
        if peeker.eat_eq(&Keyword::True).is_ok() {
            return Self::wrap_expr(Literal::Bool(true));
        }
        if peeker.eat_eq(&Keyword::False).is_ok() {
            return Self::wrap_expr(Literal::Bool(false));
        }
        Err(FrontendError::None)
    }

    fn wrap_expr(literal: Literal) -> Result<Box<Expr>> {
        Ok(Box::new(Expr::Literal(literal)))
    }
}
