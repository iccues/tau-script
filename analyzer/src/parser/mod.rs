use crate::lexer::token_peeker::TokenPeeker;
use stmt::Stmt;
use crate::error::FrontendResult as Result;

pub mod expr;
pub mod stmt;

pub fn parse_stmt(peeker: &mut TokenPeeker) -> Result<Stmt> {
    Stmt::parse(peeker)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_stmt_parser() {
        let input = Cursor::new("1 + (2 + 3);");
        let mut lexer = crate::lexer::get_lexer(input);
    
        Stmt::parse(&mut lexer).unwrap();
    }
}
