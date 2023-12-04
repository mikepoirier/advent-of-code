use aoc_2023::{run, Command};
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();
    match run(cli.command) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e)
        }
    }
}
