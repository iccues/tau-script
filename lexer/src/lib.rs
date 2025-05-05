use std::io::{BufReader, Read};

use stream::{char_stream::CharStream, peekable::peeker::Peeker, token_stream::{lexer::Lexer, token_processor::TokenProcessor}, Stream};
use token::TokenBox;

pub mod token;
pub mod stream;
// pub mod try_parse;

pub fn get_lexer(input: impl Read + 'static) -> Peeker<TokenBox> {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    TokenProcessor::new(lexer).peeker()
}