use std::io::stdin;

use frontend::lexer::get_lexer;
use frontend::parser::parse_stmt;

#[test]
#[ignore]
fn print_token() {
    let mut lexer = get_lexer(stdin());
    while let Ok(token) = lexer.next() {
        println!("{:?}", token);
    }
}

#[test]
#[ignore]
fn print_ast() {
    let mut lexer = get_lexer(stdin());
    while let Ok(stmt) = parse_stmt(&mut lexer) {
        println!("{:?}", stmt);
    }
}
