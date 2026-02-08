use colored::Colorize;
use std::process::ExitCode;

use crate::session::auth::{self, CredentialsTestResult};

use nasomail_shared::payload::auth::AuthPayload;

pub async fn login(name: String, passphrase: String) -> anyhow::Result<ExitCode> {
    auth::set_credentials(&AuthPayload {
        username: name.clone(),
        passphrase: passphrase,
    })
    .await?;

    let result = auth::try_credentials().await?;

    Ok(if result == CredentialsTestResult::Success {
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
    })
}
