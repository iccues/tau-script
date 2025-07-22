pub mod exec;

use std::io::{Read, Write};

use object::core::prelude::*;
use object::ext::tuple;
use object::types::primitive::string::ObjString;
use parser::parse_stmt;
use exec::Exec;
use object::types::env::local::ObjLocal;

pub fn execute(input: impl Read + 'static, output: &mut impl Write) {
    let mut lexer = lexer::get_lexer(input);
    let env: Object = ObjLocal::new();

    loop {
        let stmt = parse_stmt(&mut lexer);
        if let Ok(stmt) = stmt {
            let ret = stmt.exec(&env);
            print_object(ret, output);
        } else {
            println!("Error: {:?}", stmt.err().unwrap());
            break;
        }
    }
}

fn print_object(object: Object, output: &mut impl Write) -> Option<()> {
    let string = object.get_member("to_string")?.call(tuple!());
    let string: Object<ObjString> = string.downcast()?;
    writeln!(output, "{}", string.value).ok()?;
    Some(())
}
