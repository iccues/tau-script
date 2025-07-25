use frontend_library::stream::peeker::Peeker;
use frontend_library::token::{
    number::{Float, Integer},
    string::StringToken,
    TokenBox,
};

use frontend_library::error::{FrontendError, FrontendResult as Result};

use crate::expr::expr::Expr;

#[derive(Debug)]
pub enum Literal {
    Integer(String),
    Float(String),
    String(String),
    // Bool(bool),
}

impl Literal {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        if let Ok(integer) = peeker.eat_type::<Integer>() {
            return Ok(Box::new(Expr::Literal(Literal::Integer(integer.number()))));
        }
        if let Ok(float) = peeker.eat_type::<Float>() {
            return Ok(Box::new(Expr::Literal(Literal::Float(float.number()))));
        }
        if let Ok(string) = peeker.eat_type::<StringToken>() {
            return Ok(Box::new(Expr::Literal(Literal::String(string.string()))));
        }
        Err(FrontendError::None)
    }
}
