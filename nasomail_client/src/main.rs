use clap::Parser;
use cli::Cli;

pub mod auth;
pub mod cli;
pub mod meta;

fn main() {
    Cli::parse();
}
