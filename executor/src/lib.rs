pub mod exec;

use std::io::{Read, Write};

use lexer::stream::peekable::Peekable;
use object::types::{env::Env, string::String_, tuple::Tuple};
use parser::parse_stmt;
use exec::Exec;

pub fn execute(input: impl Read + 'static, output: &mut impl Write) {
    let mut lexer = lexer::get_lexer(input);
    let mut cursor = lexer.cursor();
    let env = Env::new();

    while let Ok(stmt) = parse_stmt(&mut cursor) {
        let ret = stmt.exec(&env);
        if let Some(string) = ret.get_member("to_string").call(Tuple::new(vec![])).get_data::<String_>() {
            writeln!(output, "{}", string.value).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{stdin, stdout};

    use super::*;

    #[test]
    fn test_execute() {
        execute(stdin(), &mut stdout());
    }
}
