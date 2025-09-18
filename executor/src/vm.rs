use analyzer::lexer::token_peeker::TokenPeeker;
use analyzer::parser::parse_stmt;
use analyzer::parser::stmt::Stmt;
use object::core::prelude::*;
use object::ext::core_type::string::ObjString;
use object::ext::tuple;
use object::types::env::{local::ObjLocal, prelude::prelude};

use crate::error::ExecutorResult;
use crate::exec::Exec;

pub struct VM {
    env: Object,
}

impl VM {
    pub fn new() -> Self {
        VM { env: ObjLocal::from_prelude(prelude.clone()) }
    }

    pub fn run_stmt_print(&self, lexer: &mut TokenPeeker) -> ExecutorResult<()> {
        let stmt = parse_stmt(lexer)?;
        let result = stmt.exec(&self.env)?;
        _ = self.print_object(result);
        Ok(())
    }

    pub fn exec_stmt(&self, stmt: Stmt) -> ExecutorResult<Object> {
        stmt.exec(&self.env)
    }

    fn print_object(&self, object: Object) -> ExecutorResult<()> {
        let string = object.get_member("to_string")?.try_call()?(tuple!());
        let string: Object<ObjString> = string.downcast()?;
        println!("{}", string.value);
        Ok(())
    }
}
