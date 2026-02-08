use std::process::ExitCode;

use crate::session::{
    auth::{self, CredentialsTestResult},
    connection::{self, ConnectionTestResult},
};
use clap::{Parser, Subcommand};
use colored::Colorize;

use nasomail_shared::payload::auth::AuthPayload;

/// A simple client application for communicating through a NasoMail server
#[derive(Parser)]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate as a user account specified
    /// by the `--name` flag
    LogIn {
        /// The name of the user account to log into
        #[arg(short, long)]
        name: String,

        /// The passphrase of the user account to log into
        #[arg(short, long)]
        passphrase: String,
    },

    /// Log out of the current user account
    LogOut,

    /// Connect to the specified server
    Connect {
        /// The address of the server to connect to
        addr: String,
    },

    /// Disconnect from the currently connected server
    Disconnect,
}

impl Cli {
    /// Runs the command specified by the user
    /// when parsing the arguments.
    pub async fn run(self) -> anyhow::Result<ExitCode> {
        Ok(match self.command {
            // ############
            // ## LOG IN ##
            // ############
            Commands::LogIn { name, passphrase } => {
                auth::set_credentials(&AuthPayload {
                    username: name.clone(),
                    passphrase: passphrase,
                })
                .await?;

                let result = auth::try_credentials().await?;

                if result == CredentialsTestResult::Success {
                    println!(
                        "{}{}",
                        "Success".bright_green().bold(),
                        format!(
                            ": Logged in as{}",
                            format!(": {}", name.trim()).bright_blue().bold()
                        )
                    );

                    ExitCode::SUCCESS
                } else {
                    auth::remove_credentials().await?;

                    println!(
                        "{}{}",
                        "Error".bright_red().bold(),
                        format!(
                            ": Failed to authenticate{}",
                            format!(": {:?}", result).bright_blue().bold()
                        )
                    );

                    ExitCode::FAILURE
                }
            }

            // #############
            // ## LOG OUT ##
            // #############
            Commands::LogOut => {
                auth::remove_credentials().await?;

                println!("{}{}", "Success".bright_green().bold(), ": Logged out");

                ExitCode::SUCCESS
            }

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

                    ExitCode::SUCCESS
                } else {
                    connection::remove_connection().await?;

                    println!(
                        "{}{}",
                        "Error".bright_red().bold(),
                        format!(
                            ": Could not reach server{}{}",
                            format!(": {}", addr.trim()).bright_blue().bold(),
                            format!(": {:?}", result)
                        )
                    );

                    ExitCode::FAILURE
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

                    ExitCode::SUCCESS
                } else {
                    println!(
                        "{}{}",
                        "Warning".bright_yellow().bold(),
                        ": No server is currently connected"
                    );

                    ExitCode::FAILURE
                }
            }
        }) // <------+
        //           |
        //   ###############
        //   ## MATCH END ##
        //   ###############
    }
}
