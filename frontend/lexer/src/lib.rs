use std::io::{BufReader, Read};

use frontend_library::stream::{peeker::Peeker, Stream};
use char_stream::CharStream;
use frontend_library::token::TokenBox;
use token_stream::{lexer::Lexer, token_processor::TokenProcessor};

pub mod token_stream;
pub mod char_stream;

pub fn get_lexer(input: impl Read + 'static) -> Peeker<TokenBox> {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    TokenProcessor::new(lexer).peeker()
}
