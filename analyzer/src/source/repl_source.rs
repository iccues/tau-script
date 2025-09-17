use std::io::{BufRead, BufReader, Read};
use std::sync::{Mutex, RwLock};

use crate::{error::FrontendResult, source::{Source, EOF_CHAR}};

pub struct ReplSource {
    input: Mutex<Box<dyn BufRead>>,
    buffer: RwLock<String>,
}

impl ReplSource {
    pub fn new(reader: impl Read + 'static) -> Self {
        Self {
            input: Mutex::new(Box::new(BufReader::new(reader))),
            buffer: RwLock::new(String::new()),
        }
    }

    fn input(&self) -> std::sync::MutexGuard<'_, Box<dyn BufRead>> {
        self.input.lock().unwrap()
    }
    fn buffer(&self) -> std::sync::RwLockReadGuard<'_, String> {
        self.buffer.read().unwrap()
    }
    fn buffer_mut(&self) -> std::sync::RwLockWriteGuard<'_, String> {
        self.buffer.write().unwrap()
    }


    fn load(&self, index: usize) -> FrontendResult<()> {
        loop {
            if self.buffer().get_char(index)? != EOF_CHAR {
                break;
            }

            if self.input().read_line(&mut self.buffer_mut())? == 0 {
                break;
            }
        }
        Ok(())
    }
}

impl Source for ReplSource {
    fn get_char(&self, index: usize) -> FrontendResult<char> {
        self.load(index)?;
        self.buffer().get_char(index)
    }

    fn next_index(&self, index: usize) -> FrontendResult<usize> {
        self.load(index)?;
        self.buffer().next_index(index)
    }
}
