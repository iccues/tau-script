pub mod lexer;
pub mod token_peeker;

use std::io::{BufReader, Read};
use crate::library::stream::Stream;
use crate::source::CharStream;
use lexer::Lexer;
use crate::lexer::token_peeker::TokenPeeker;


pub fn get_lexer(input: impl Read + 'static) -> TokenPeeker {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = TokenPeeker::new(Lexer::new(char_stream));

    lexer
}
