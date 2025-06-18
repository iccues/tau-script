use std::rc::Rc;

use crate::Token;
use crate::TokenBox;


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