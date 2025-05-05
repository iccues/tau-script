use std::io::{BufReader, Read};

use stream::{
    Stream,
    char_stream::CharStream,
    peekable::peeker::Peeker,
    token_stream::{lexer::Lexer, token_processor::TokenProcessor},
};
use token::TokenBox;

pub mod stream;
pub mod token;

pub fn get_lexer(input: impl Read + 'static) -> Peeker<TokenBox> {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    TokenProcessor::new(lexer).peeker()
}
