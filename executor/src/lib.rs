pub mod exec;

use std::io::{Read, Write};

use object::types::env::build_in::BuildIn;
use object::types::primitive::string::String_;
use parser::parse_stmt;
use exec::Exec;
use object::types::compound::tuple::Tuple;
use object::types::env::local::Local;

pub fn execute(input: impl Read + 'static, output: &mut impl Write) {
    let mut lexer = lexer::get_lexer(input);
    let env = Local::from_outer(vec![BuildIn::new()]);

    loop {
        let stmt = parse_stmt(&mut lexer);
        if let Ok(stmt) = stmt {
            let ret = stmt.exec(&env);
            if let Some(string) = ret.get_member("to_string").call(Tuple::new(vec![])).get_data::<String_>() {
                writeln!(output, "{}", string.value).unwrap();
            }
        } else {
            println!("Error: {:?}", stmt.err().unwrap());
            break;
        }
    }
}
