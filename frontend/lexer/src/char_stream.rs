use std::io::{BufRead, Lines};
use std::rc::Rc;
use frontend_library::error::FrontendResult as Result;

use frontend_library::stream::{Position, Stream};


pub const EOF_CHAR: char = '\0';

pub struct CharStream {
    reader: Lines<Box<dyn BufRead>>,
    line: Vec<char>,
    pub position: Position,
}

impl CharStream {
    pub fn new(reader: impl BufRead + 'static) -> Self {
        Self {
            reader: (Box::new(reader) as Box<dyn BufRead>).lines(),
            line: vec![],
            position: Position::new(),
        }
    }

    pub fn next(&mut self) -> Result<char> {
        self.position.move_right();
        if self.position.column >= self.line.len() {
            match self.reader.next() {
                Some(Ok(line)) => {
                    self.line = line.chars().collect();
                    self.line.push('\n');
                    self.position.move_down();
                }
                Some(Err(err)) => return Err(Rc::new(err).into()),
                None => return Ok(EOF_CHAR),
            }
        }
        Ok(self.line[self.position.column])
    }
}

impl Stream for CharStream {
    type Item = char;

    fn next(&mut self) -> Result<Self::Item> {
        self.next()
    }

    fn last_position(&self) -> Position {
        self.position
    }
    fn next_position(&self) -> Position {
        self.position
    }
}
