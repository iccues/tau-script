use std::collections::VecDeque;
use crate::error::*;
use crate::token::{Token, TokenBox};

use crate::lexer::lexer::Lexer;

pub struct TokenPeeker<'src> {
    inner: Lexer<'src>,
    buffer: VecDeque<Result<TokenBox>>,
}

impl TokenPeeker<'_> {
    pub fn new<'src>(inner: Lexer<'src>) -> TokenPeeker<'src> {
        TokenPeeker {
            inner,
            buffer: VecDeque::new(),
        }
    }

    fn get_next(&mut self) {
        let item = self.inner.next();
        self.buffer.push_back(item);
    }

    pub fn peek(&mut self) -> Result<TokenBox> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer[0].clone()
    }

    pub fn next(&mut self) -> Result<TokenBox> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer.pop_front().unwrap()
    }

    pub fn eat_type<T: Token>(&mut self) -> Result<TokenBox<T>> {
        let item = self.peek()?.downcast()?;
        let _ = self.next();
        Ok(item)
    }

    pub fn eat_eq(&mut self, value: &dyn Token) -> Result<()> {
        if self.peek()?.eq(value) {
            let _ = self.next();
            Ok(())
        } else {
            Err(AnalyzerError::None)
        }
    }
}
