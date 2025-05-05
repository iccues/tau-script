use std::io::{BufReader, Read};

use lexer::stream::peekable::cursor::Cursor;
use lexer::token::TokenBox;
use signal_table::func::stmt::Stmt;
use signal_table::SignalTable;
use lexer::stream::char_stream::CharStream;
use lexer::stream::peekable::Peekable;
use lexer::stream::Stream;
use lexer::token::singleton_token::EofToken;
use lexer::stream::token_stream::lexer::Lexer;
use lexer::stream::token_stream::token_processor::TokenProcessor;

#[macro_use]
pub mod signal_table;


pub fn parser(input: impl Read + 'static) -> SignalTable {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    let mut token_processor = TokenProcessor::new(lexer).peeker();

    let mut cursor = token_processor.cursor();

    SignalTable::parse(&mut cursor).unwrap()
}

pub fn parse_stmt(cursor: &mut Cursor<'_, TokenBox>) -> error::Result<Stmt> {
    Stmt::parse(cursor)
}

#[allow(dead_code)]
pub fn print_token(input: impl Read + 'static) {
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    let mut token_processor = TokenProcessor::new(lexer).peeker();

    loop {
        let token = token_processor.next().unwrap();
        if token.is::<EofToken>() {
            break;
        }
        println!("{:?}", token);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_stmt_parser() {
        let input = Cursor::new("1 + (2 + 3);");

        let char_stream = CharStream::new(BufReader::new(input)).peeker();
        let lexer = Lexer::new(char_stream).peeker();
        let mut token_processor = TokenProcessor::new(lexer).peeker();
    
        let mut cursor = token_processor.cursor();
    
        Stmt::parse(&mut cursor).unwrap();
        // Stmt::parse(&mut cursor).unwrap();
    }
}