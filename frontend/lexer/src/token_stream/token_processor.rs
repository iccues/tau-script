use frontend_library::error::FrontendResult as Result;

use frontend_library::stream::{peeker::Peeker, Stream};
use frontend_library::token::TokenBox;
use frontend_library::token::keyword::Keyword;

pub struct TokenProcessor {
    input: Peeker<TokenBox>,
}

impl Stream for TokenProcessor {
    type Item = TokenBox;

    fn next(&mut self) -> Result<Self::Item> {
        self.next_token()
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
