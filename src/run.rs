use analyzer::{lexer::get_lexer, source::{file_source::FileSource, repl_source::ReplSource}};
use executor::vm::VM;
use std::io::stdin;


pub fn run_repl() {
    let vm = VM::new();

    let source = ReplSource::new(stdin());
    let mut lexer = get_lexer(&source);

    loop {
        if let Err(err) = vm.run_stmt_print(&mut lexer) {
            println!("{}", err);
            break;
        }
    }
}

pub fn run_file(path: String) {
    let vm = VM::new();

    let source = FileSource::new(&path).unwrap();
    let mut lexer = get_lexer(&source);

    loop {
        if let Err(err) = vm.run_stmt_print(&mut lexer) {
            println!("{}", err);
            break;
        }
    }
}
