use crate::error::Result;
use crate::token::Token;
use crate::token::TokenBox;


#[derive(Debug, PartialEq)]
pub struct StringToken {
    string: String,
}

impl StringToken {
    pub fn new(string: String) -> Result<TokenBox> {
        let string = string[1..string.len() - 1].to_string();
        Ok(TokenBox::new(Self { string }))
    }

    pub fn string(&self) -> String {
        self.string.clone()
    }
}

impl Token for StringToken {}
