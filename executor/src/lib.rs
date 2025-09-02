pub mod exec;
pub mod error;

use std::io::{Read, Write};

use frontend::frontend_library::stream::peeker::Peeker;
use frontend::frontend_library::token::TokenBox;
use object::core::prelude::*;
use object::ext::tuple;
use object::types::env::prelude;
use object::ext::core_type::string::ObjString;
use frontend::parser::parse_stmt;
use exec::Exec;
use object::types::env::local::ObjLocal;

use crate::error::ExecutorResult;

pub fn execute(input: impl Read + 'static, output: &mut impl Write) {
    let mut lexer = frontend::lexer::get_lexer(input);
    let env: Object = ObjLocal::from_prelude(prelude::prelude.clone());

    loop {
        if let Err(err) = execute_stmt(&mut lexer, output, &env) {
            println!("Error: {}", err);
            break;
        }
    }
}

fn execute_stmt(lexer: &mut Peeker<TokenBox>, output: &mut impl Write, env: &Object) -> ExecutorResult<()> {
    let stmt = parse_stmt(lexer)?;
    let result = stmt.exec(env)?;
    _ = print_object(result, output);
    Ok(())
}

fn print_object(object: Object, output: &mut impl Write) -> ExecutorResult<()> {
    let string = object.get_member("to_string")?.try_call()?(tuple!());
    let string: Object<ObjString> = string.downcast()?;
    writeln!(output, "{}", string.value)?;
    Ok(())
}
