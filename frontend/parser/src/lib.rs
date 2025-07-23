use frontend_library::stream::peeker::Peeker;
use frontend_library::token::TokenBox;
use stmt::Stmt;
use frontend_library::error::FrontendResult as Result;

pub mod expr;
pub mod stmt;

pub fn parse_stmt(peeker: &mut Peeker<TokenBox>) -> Result<Stmt> {
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
