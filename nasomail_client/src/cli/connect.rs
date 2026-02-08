use colored::Colorize;
use std::process::ExitCode;

use crate::session::connection::{self, ConnectionTestResult};

pub async fn connect(addr: String) -> anyhow::Result<ExitCode> {
    connection::set_connection(&addr).await?;

    let result = connection::try_connection().await?;

    Ok(if result == ConnectionTestResult::Success {
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
    })
}
