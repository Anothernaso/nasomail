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
        // ############
        // ## LOG IN ##
        // ############
        Commands::LogIn {
            name: _,
            passphrase: _,
        } => {}

        // #############
        // ## LOG OUT ##
        // #############
        Commands::LogOut => {}

        // #############
        // ## CONNECT ##
        // #############
        Commands::Connect { addr } => {
            connection::set_connection(&addr).await?;

            let result = connection::try_connection().await?;

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

        // ################
        // ## DISCONNECT ##
        // ################
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
    } // <------+
    //          |
    //   ###############
    //   ## MATCH END ##
    //   ###############

    exit(exit_code);
}
