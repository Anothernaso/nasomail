use colored::Colorize;
use std::process::ExitCode;

use crate::session::auth;

pub async fn logout() -> anyhow::Result<ExitCode> {
    auth::remove_credentials().await?;

    println!("{}{}", "Success".bright_green().bold(), ": Logged out");

    Ok(ExitCode::SUCCESS)
}
