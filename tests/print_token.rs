use std::io::stdin;

use lexer::get_lexer;
use parser::parse_stmt;

#[test]
fn print_token() {
    let mut lexer = get_lexer(stdin());
    while let Ok(token) = lexer.next() {
        println!("{:?}", token);
    }
}

#[test]
fn print_ast() {
    let mut lexer = get_lexer(stdin());
    while let Ok(stmt) = parse_stmt(&mut lexer) {
        println!("{:?}", stmt);
    }
}
