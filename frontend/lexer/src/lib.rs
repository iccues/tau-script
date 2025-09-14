pub mod lexer;
pub mod char_stream;
pub mod token_peeker;


use std::io::{BufReader, Read};
use frontend_library::stream::Stream;
use char_stream::CharStream;
use lexer::Lexer;
use crate::token_peeker::TokenPeeker;


pub fn get_lexer(input: impl Read + 'static) -> TokenPeeker {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = TokenPeeker::new(Lexer::new(char_stream));

    lexer
}
