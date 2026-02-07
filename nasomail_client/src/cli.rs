use clap::{Parser, Subcommand};

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
