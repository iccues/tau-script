mod args;
mod run;

use clap::Parser;

use crate::args::Args;
use crate::run::{run_repl, run_file};


fn main() {
    println!("Hello from tau-script!");

    let args = Args::parse();

    if let Some(path) = args.src {
        run_file(path);
    } else {
        run_repl();
    }
}
