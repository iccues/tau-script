pub mod exec;

use std::io::stdin;

use lexer::stream::peekable::Peekable;
use object::types::{env::Env, string::String_, tuple::Tuple};
use parser::parse_stmt;
use exec::Exec;

pub fn execute() {
    let mut lexer = lexer::get_lexer(stdin());
    let mut cursor = lexer.cursor();
    let env = Env::new();

    while let Ok(stmt) = parse_stmt(&mut cursor) {
        let ret = stmt.exec(&env);
        if let Some(string) = ret.get_member("to_string").call(Tuple::new(vec![])).get_data::<String_>() {
            println!("{}", string.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        execute();
    }
}
