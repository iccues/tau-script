use crate::{error::FrontendResult, source::{Source, EOF_CHAR}};


impl Source for String {
    fn get_char(&self, index: usize) -> FrontendResult<char> {
        if index >= self.len() {
            return Ok(EOF_CHAR);
        }

        let mut chars = self[index..].chars();
        Ok(chars.next().unwrap_or(EOF_CHAR))
    }

    fn next_index(&self, index: usize) -> FrontendResult<usize> {
        if index >= self.len() {
            return Ok(index);
        }

        let mut chars = self[index..].chars();
        Ok(index + chars.next().map_or(0, |ch| ch.len_utf8()))
    }
}
