use std::sync::LazyLock;

use crate::token::Token;
use crate::token::TokenBox;

#[derive(Debug, PartialEq)]
pub struct EofToken;

const EOF_TOKEN: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(EofToken));

impl EofToken {
    pub fn new() -> TokenBox {
        EOF_TOKEN.clone()
    }
}

impl Token for EofToken {}
