mod cli;
mod meta;
mod session;

use clap::Parser;
use std::process::{ExitCode, exit};
use tokio::task;

use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let cli = task::spawn_blocking(|| Cli::parse()).await?;
    cli.run().await
}
