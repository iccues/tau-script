use crate::Token;
use crate::TokenBox;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Integer {
    number: String,
}

impl Integer {
    pub fn new(num: String) -> TokenBox {
        TokenBox::Rc(Rc::new(Integer { number: num }))
    }

    pub fn number(&self) -> String {
        self.number.clone()
    }
}

impl Token for Integer {}


#[derive(Debug, PartialEq)]
pub struct Float {
    number: String,
}

impl Float {
    pub fn new(num: String) -> TokenBox {
        TokenBox::Rc(Rc::new(Float { number: num }))
    }

    pub fn number(&self) -> String {
        self.number.clone()
    }
}

impl Token for Float {}