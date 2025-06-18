use std::rc::Rc;

use crate::Token;
use crate::TokenBox;

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