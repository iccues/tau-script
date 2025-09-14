use std::collections::VecDeque;

use frontend_library::{error::{FrontendError, FrontendResult}, token::{Token, TokenBox}};

use crate::lexer::Lexer;

pub struct TokenPeeker {
    inner: Lexer,
    buffer: VecDeque<FrontendResult<TokenBox>>,
}

impl TokenPeeker {
    pub fn new(inner: Lexer) -> Self {
        Self {
            inner,
            buffer: VecDeque::new(),
        }
    }

    fn get_next(&mut self) {
        let item = self.inner.next();
        self.buffer.push_back(item);
    }

    pub fn peek(&mut self) -> FrontendResult<TokenBox> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer[0].clone()
    }

    pub fn next(&mut self) -> FrontendResult<TokenBox> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer.pop_front().unwrap()
    }

    pub fn eat_type<T: Token>(&mut self) -> FrontendResult<TokenBox<T>> {
        let item = self.peek()?.downcast()?;
        let _ = self.next();
        Ok(item)
    }

    pub fn eat_eq(&mut self, value: &dyn Token) -> FrontendResult<()> {
        if self.peek()?.eq(value) {
            let _ = self.next();
            Ok(())
        } else {
            Err(FrontendError::None)
        }
    }
}
