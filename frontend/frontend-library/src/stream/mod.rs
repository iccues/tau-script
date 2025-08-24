use peeker::Peeker;

use crate::error::FrontendResult as Result;

pub mod peeker;

pub trait Stream {
    type Item: Clone;

    fn next(&mut self) -> Result<Self::Item>;
    fn nexts(&mut self, n: usize) -> Result<Vec<Self::Item>> {
        let mut items = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(self.next()?);
        }
        Ok(items)
    }

    fn peeker(self) -> Peeker<Self::Item> where Self: Sized + 'static {
        Peeker::new(self)
    }

    fn last_position(&self) -> Position;
    fn next_position(&self) -> Position;
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn move_right(&mut self) {
        self.column += 1;
    }

    pub fn move_down(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}
