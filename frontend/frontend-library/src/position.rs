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
