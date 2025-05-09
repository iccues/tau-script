use error::{try_parse, Result};

use crate::stream::{peeker::Peeker, Stream};
use crate::token::TokenBox;
use crate::token::keyword::Keyword;

pub struct TokenProcessor {
    input: Peeker<TokenBox>,
}

impl Stream for TokenProcessor {
    type Item = TokenBox;

    fn next(&mut self) -> Result<Self::Item> {
        self.next_token()
    }

    fn last_position(&self) -> crate::stream::Position {
        self.input.last_position()
    }
    fn next_position(&self) -> crate::stream::Position {
        self.input.next_position()
    }
}

impl TokenProcessor {
    pub fn new(input: Peeker<TokenBox>) -> Self {
        Self { input }
    }

    pub fn next_token(&mut self) -> Result<TokenBox> {

        try_parse!(Keyword::parse(&mut self.input));

        self.input.next()
    }
}