use crate::token::Token;
use crate::token::TokenBox;

#[derive(Debug, PartialEq)]
pub struct EofToken;

const EOF_TOKEN: EofToken = EofToken;

impl EofToken {
    pub fn new() -> TokenBox {
        TokenBox::Ref(&EOF_TOKEN)
    }
}

impl Token for EofToken {}
