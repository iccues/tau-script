pub mod exec;

use std::io::stdin;

use interpreter::object::env::Env;
use lexer::stream::peekable::Peekable;
use parser::parse_stmt;

use exec::Exec;

pub fn execute() {
    let mut lexer = lexer::get_lexer(stdin());
    let mut cursor = lexer.cursor();
    let env = Env::new();

    while let Ok(stmt) = parse_stmt(&mut cursor) {
        println!("{}", stmt.exec(&env).to_string_row());
    }
}
