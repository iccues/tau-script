use lexer::stream::peekable::cursor::Cursor;
use lexer::token::TokenBox;
use stmt::Stmt;

pub mod expr;
pub mod stmt;

pub fn parse_stmt(cursor: &mut Cursor<'_, TokenBox>) -> error::Result<Stmt> {
    Stmt::parse(cursor)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use lexer::stream::peekable::Peekable;

    use super::*;

    #[test]
    fn test_stmt_parser() {
        let input = Cursor::new("1 + (2 + 3);");
        let mut lexer = lexer::get_lexer(input);
        let mut cursor = lexer.cursor();
    
        Stmt::parse(&mut cursor).unwrap();
        // Stmt::parse(&mut cursor).unwrap();
    }
}
