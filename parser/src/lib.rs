use stream::peeker::Peeker;
use token::TokenBox;
use stmt::Stmt;

pub mod expr;
pub mod stmt;

pub fn parse_stmt(peeker: &mut Peeker<TokenBox>) -> error::Result<Stmt> {
    Stmt::parse(peeker)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_stmt_parser() {
        let input = Cursor::new("1 + (2 + 3);");
        let mut lexer = lexer::get_lexer(input);
    
        Stmt::parse(&mut lexer).unwrap();
    }
}
