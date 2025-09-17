pub mod cursor;
pub mod str_source;
pub mod repl_source;


use crate::error::FrontendResult as Result;


pub const EOF_CHAR: char = '\0';

pub trait Source {
    fn get_char(&self, index: usize) -> Result<char>;
    fn next_index(&self, index: usize) -> Result<usize>;
    fn next(&self, index: usize) -> Result<(char, usize)> {
        Ok((self.get_char(index)?, self.next_index(index)?))
    }
}
