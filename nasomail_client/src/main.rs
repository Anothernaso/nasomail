use std::process::exit;

use clap::Parser;
use colored::Colorize;
use tokio::task;

mod auth;
mod cli;
mod connection;
mod meta;

use cli::{Cli, Commands};

use crate::connection::ConnectionTestResult;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = task::spawn_blocking(|| Cli::parse()).await?;

    let mut exit_code = 0;

    match cli.command {
        Commands::LogIn {
            name: _,
            passphrase: _,
        } => {}
        Commands::LogOut => {}
        Commands::Connect { addr } => {
            connection::set_connection(&addr).await?;

            let result = connection::test_connection().await?;

            if result == ConnectionTestResult::Success {
                println!(
                    "{}{}",
                    "Success".bright_green().bold(),
                    format!(
                        ": Connected to server{}",
                        format!(": {}", addr.trim()).bright_blue().bold()
                    )
                );
            } else {
                connection::remove_connection().await?;

                exit_code = 1;
                println!(
                    "{}{}",
                    "Error".bright_red().bold(),
                    format!(
                        ": Could not reach server{}{}",
                        format!(": {}", addr.trim()).bright_blue().bold(),
                        format!(": {:?}", result)
                    )
                );
            }
        }
        Commands::Disconnect => {
            if connection::remove_connection().await? {
                println!(
                    "{}{}",
                    "Success".bright_green().bold(),
                    ": Disconnected from server"
                );
            } else {
                println!(
                    "{}{}",
                    "Warning".bright_yellow().bold(),
                    ": No server is currently connected"
                );
            }
        }
    }

    exit(exit_code);
}
