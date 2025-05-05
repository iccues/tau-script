#![allow(dead_code)]

pub mod types;
pub mod variable;
pub mod module;
pub mod func;
pub mod path;

use module::Module;

use lexer::{stream::peekable::cursor::Cursor, token::{singleton_token::EofToken, TokenBox}};
use error::Result;

#[derive(Debug)]
pub struct SignalTable {
    pub inner: Module,
}

impl SignalTable {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<SignalTable> {
        let inner = Module::parser_inner(cursor)?;
        cursor.eat_eq(&EofToken)?;
        Ok(SignalTable { inner, })
    }
}
