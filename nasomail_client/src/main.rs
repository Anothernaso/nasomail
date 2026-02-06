use clap::Parser;
use cli::Cli;

pub mod cli;

fn main() {
    Cli::parse();
}
