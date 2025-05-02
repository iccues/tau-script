use executor::execute;
use std::io::{stdin, stdout};

fn main() {
    println!("Hello from tau-script!\n");
    execute(stdin(), &mut stdout());
}