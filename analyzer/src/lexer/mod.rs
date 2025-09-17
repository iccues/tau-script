pub mod lexer;
pub mod token_peeker;

use crate::source::cursor::Cursor;
use crate::source::Source;
use lexer::Lexer;
use crate::lexer::token_peeker::TokenPeeker;


pub fn get_lexer<'src>(input: &'src dyn Source) -> TokenPeeker<'src> {
    let cursor = Cursor::new(input);
    let lexer = TokenPeeker::new(Lexer::new(cursor));

    lexer
}
