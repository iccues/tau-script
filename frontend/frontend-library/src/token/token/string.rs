use crate::error::FrontendResult;
use crate::token::Token;
use crate::token::TokenBox;
use std::rc::Rc;


#[derive(Debug, PartialEq)]
pub struct StringToken {
    string: String,
}

impl StringToken {
    pub fn new(string: String) -> FrontendResult<TokenBox> {
        let string = string[1..string.len() - 1].to_string();
        Ok(TokenBox::Rc(Rc::new(Self { string })))
    }

    pub fn string(&self) -> String {
        self.string.clone()
    }
}

impl Token for StringToken {}