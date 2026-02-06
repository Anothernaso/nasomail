use clap::Parser;
use cli::Cli;
use tokio::task;

pub mod cli;
pub mod meta;

#[tokio::main]
async fn main() {
    task::spawn_blocking(|| {
        Cli::parse();
    })
    .await
    .expect("cli task failed");
}
