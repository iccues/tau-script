use std::collections::VecDeque;

use error::{NoneError, Result};
use super::{Position, Stream};
use token::{Token, TokenBox};

pub struct Peeker<I: Clone> {
    inner: Box<dyn Stream<Item = I>>,
    buffer: VecDeque<(Position, Position, Result<I>)>,
}

impl<I: Clone> Stream for Peeker<I> {
    type Item = I;

    fn next(&mut self) -> Result<Self::Item> {
        self.next()
    }

    fn last_position(&self) -> Position {
        if self.buffer.is_empty() {
            self.inner.last_position()
        } else {
            self.buffer[0].0
        }
    }
    fn next_position(&self) -> Position {
        if self.buffer.is_empty() {
            self.inner.next_position()
        } else {
            self.buffer[0].1
        }
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
        let last_position = self.inner.last_position();
        let next_position = self.inner.next_position();
        let item = self.inner.next();
        self.buffer.push_back((last_position, next_position, item));
    }

    pub fn peek(&mut self) -> Result<I> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer[0].2.clone()
    }

    pub fn peek_n(&mut self, n: usize) -> Result<I> {
        for _ in self.buffer.len()..n + 1 {
            self.get_next();
        }
        self.buffer[n].2.clone()
    }

    pub fn peeks(&mut self, n: usize) -> Result<Vec<I>> {
        for _ in self.buffer.len()..n {
            self.get_next();
        }

        self.buffer
            .iter()
            .take(n)
            .map(|x| x.2.clone())
            .collect::<Result<Vec<I>>>()
    }

    pub fn next(&mut self) -> Result<I> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer.pop_front().unwrap().2
    }
}

impl Peeker<char> {
    pub fn peek_str(&mut self, n: usize) -> Result<String> {
        Ok(self.peeks(n)?.into_iter().collect())
    }

    pub fn eat_str(&mut self, s: &str) -> Result<()> {
        if self.peek_str(s.len())? == s {
            self.nexts(s.len())?;
            Ok(())
        } else {
            Err(NoneError.into())
        }
    }

    pub fn eat_whitespace(&mut self) -> Result<()> {
        while self.peek()?.is_whitespace() {
            self.next()?;
        }
        Ok(())
    }
}

impl Peeker<TokenBox> {
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
            Err(NoneError.into())
        }
    }
}
