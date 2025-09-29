use tau_script;

use clap::Parser;

use tau_script::args::Args;
use tau_script::run::{run_repl, run_file};


fn main() {
    println!("Hello from tau-script!");

    let args = Args::parse();

    if let Some(path) = args.src {
        run_file(path);
    } else {
        run_repl();
    }
}
