
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
pub struct Cli {

    #[clap(long, short ='d', value_parser, help = "Maximum depth to be browsed")]
    pub depth: i32, 
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
