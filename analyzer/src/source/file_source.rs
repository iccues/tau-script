use std::{fs::File, io::Read};

use crate::{error::FrontendResult, source::Source};

pub struct FileSource {
    buffer: String
}

impl FileSource {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(Self { buffer })
    }
}

impl Source for FileSource {
    fn get_char(&self, index: usize) -> FrontendResult<char> {
        self.buffer.get_char(index)
    }

    fn next_index(&self, index: usize) -> FrontendResult<usize> {
        self.buffer.next_index(index)
    }
}
