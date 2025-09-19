use clap::Parser;


#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    pub src: Option<String>
}
