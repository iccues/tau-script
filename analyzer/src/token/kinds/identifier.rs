use std::rc::Rc;

use crate::token::Token;
use crate::token::TokenBox;


#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl Token for Identifier {}

impl Identifier {
    pub fn new(name: String) -> TokenBox {
        TokenBox::Rc(Rc::new(Identifier { name }))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}