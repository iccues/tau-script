use std::io::{BufRead, Lines};
use frontend_library::error::FrontendResult as Result;
use frontend_library::position::Position;
use frontend_library::stream::Stream;


pub const EOF_CHAR: char = '\0';

pub struct CharStream {
    reader: Lines<Box<dyn BufRead>>,
    line: Vec<char>,
    position: Position,
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
            if let Some(line) = self.reader.next() {
                self.line = line?.chars().collect();
                self.line.push('\n');
                self.position.move_down();
            } else {
                return Ok(EOF_CHAR);
            }
        }
        Ok(self.line[self.position.column])
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

impl Stream for CharStream {
    type Item = char;

    fn next(&mut self) -> Result<Self::Item> {
        self.next()
    }
}
