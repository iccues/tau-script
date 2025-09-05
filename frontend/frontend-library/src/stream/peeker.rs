use std::collections::VecDeque;

use super::Stream;
use crate::{error::{FrontendError, FrontendResult}, token::{Token, TokenBox}};

pub struct Peeker<I: Clone> {
    inner: Box<dyn Stream<Item = I>>,
    buffer: VecDeque<FrontendResult<I>>,
}

impl<I: Clone> Stream for Peeker<I> {
    type Item = I;

    fn next(&mut self) -> FrontendResult<Self::Item> {
        self.next()
    }
}

impl<I: Clone> Peeker<I> {
    pub fn new(inner: impl Stream<Item = I> + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            buffer: VecDeque::new(),
        }
    }

    fn get_next(&mut self) {
        let item = self.inner.next();
        self.buffer.push_back(item);
    }

    pub fn peek(&mut self) -> FrontendResult<I> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer[0].clone()
    }

    pub fn peek_n(&mut self, n: usize) -> FrontendResult<I> {
        for _ in self.buffer.len()..n + 1 {
            self.get_next();
        }
        self.buffer[n].clone()
    }

    pub fn peeks(&mut self, n: usize) -> FrontendResult<Vec<I>> {
        for _ in self.buffer.len()..n {
            self.get_next();
        }

        self.buffer
            .iter()
            .take(n)
            .map(|x| x.clone())
            .collect::<FrontendResult<Vec<I>>>()
    }

    pub fn next(&mut self) -> FrontendResult<I> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer.pop_front().unwrap()
    }
}

impl Peeker<char> {
    pub fn peek_str(&mut self, n: usize) -> FrontendResult<String> {
        Ok(self.peeks(n)?.into_iter().collect())
    }

    pub fn eat_str(&mut self, s: &str) -> FrontendResult<()> {
        if self.peek_str(s.len())? == s {
            self.nexts(s.len())?;
            Ok(())
        } else {
            Err(FrontendError::None)
        }
    }

    pub fn eat_whitespace(&mut self) -> FrontendResult<()> {
        while self.peek()?.is_whitespace() {
            self.next()?;
        }
        Ok(())
    }
}

impl Peeker<TokenBox> {
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
