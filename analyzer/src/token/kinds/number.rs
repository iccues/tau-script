use crate::error::FrontendResult as Result;
use crate::token::Token;
use crate::token::TokenBox;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Integer {
    number: i64,
}

impl Integer {
    pub fn new(number: String) -> Result<TokenBox> {
        let number = number.parse()?;
        Ok(TokenBox::Rc(Rc::new(Integer { number })))
    }

    pub fn number(&self) -> i64 {
        self.number
    }
}

impl Token for Integer {}


#[derive(Debug, PartialEq)]
pub struct Float {
    number: f64,
}

impl Float {
    pub fn new(number: String) -> Result<TokenBox> {
        let number = number.parse()?;
        Ok(TokenBox::Rc(Rc::new(Float { number })))
    }

    pub fn number(&self) -> f64 {
        self.number
    }
}

impl Token for Float {}