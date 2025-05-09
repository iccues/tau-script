use error::Result;

use stream::Position;
use stream::{peeker::Peeker, Stream};
use token::TokenBox;
use token::keyword::Keyword;

pub struct TokenProcessor {
    input: Peeker<TokenBox>,
}

impl Stream for TokenProcessor {
    type Item = TokenBox;

    fn next(&mut self) -> Result<Self::Item> {
        self.next_token()
    }

    fn last_position(&self) -> Position {
        self.input.last_position()
    }
    fn next_position(&self) -> Position {
        self.input.next_position()
    }
}

impl TokenProcessor {
    pub fn new(input: Peeker<TokenBox>) -> Self {
        Self { input }
    }

    pub fn next_token(&mut self) -> Result<TokenBox> {

        if let Ok(token) = Keyword::parse(self.input.peek()?) {
            self.input.next()?;
            return Ok(token);
        }

        self.input.next()
    }
}