use std::process::exit;

use clap::Parser;
use colored::Colorize;
use tokio::task;

mod auth;
mod cli;
mod connection;
mod meta;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = task::spawn_blocking(|| Cli::parse()).await?;

    let mut exit_code = 0;

    match cli.command {
        Commands::LogIn { name, passphrase } => {}
        Commands::LogOut => {}
        Commands::Connect { addr } => {
            connection::set_connection(&addr).await?;

            let success = connection::test_connection().await?;

            if success {
                println!(
                    "{}{}",
                    "Success: ".green(),
                    format!("Connected to server: {}", addr.blue())
                );
            } else {
                connection::remove_connection().await?;

                exit_code = 1;
                println!(
                    "{}{}",
                    "Error: ".red(),
                    format!("Could not reach server: {}", addr.blue())
                );
            }
        }
    }

    exit(exit_code);
}
