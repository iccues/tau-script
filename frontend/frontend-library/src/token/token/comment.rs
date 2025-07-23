use std::rc::Rc;

use crate::token::Token;
use crate::token::TokenBox;

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub content: Option<String>,
}

impl Comment {
    pub fn new(content: Option<String>) -> TokenBox {
        TokenBox::Rc(Rc::new(Comment { content }))
    }
}

impl Token for Comment {}