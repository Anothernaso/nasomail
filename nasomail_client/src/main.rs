use clap::Parser;
use cli::Cli;
use tokio::task;

mod auth;
mod cli;
mod connection;
mod meta;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _cli = task::spawn_blocking(|| Cli::parse()).await?;

    Ok(())
}
