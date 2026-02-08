use colored::Colorize;
use std::process::ExitCode;

use crate::session::connection;

pub async fn disconnect() -> anyhow::Result<ExitCode> {
    Ok(if connection::remove_connection().await? {
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
    })
}
