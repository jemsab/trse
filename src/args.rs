
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
pub struct Cli {

    #[clap(long, short ='d', value_parser = depth_positive_value, default_value_t = 1, help = "Maximum depth to be browsed, must greater than or equal to 1")]
    pub depth: u32, 

    #[clap(value_parser, default_value_t = String::from("."), help = "Root directory to start with")]
    pub directory: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}


fn depth_positive_value(s: &str) -> Result<u32, String> {
    let depth: u32 = s.parse().map_err(|_| format!("{} isn't a proper depth", s))?;
    if depth > 0 {
        Ok(depth)
    } else {
        Err(format!("Depth must be greater than or equal to 1, got {}", depth))
    }
}
