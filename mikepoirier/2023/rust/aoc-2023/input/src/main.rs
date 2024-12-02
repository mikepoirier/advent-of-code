use clap::Parser;

use input::{save_input, Result};

#[derive(Debug, Parser)]
struct Cli {
    day: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let stdin = std::io::stdin().lock();
    let input = std::io::read_to_string(stdin)?;
    save_input(cli.day, input)?;
    Ok(())
}
