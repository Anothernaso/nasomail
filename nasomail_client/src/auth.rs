//! A utility for reading and writing the
//! user's current session.

use std::path::PathBuf;

use anyhow::Ok;
use nasomail_shared::auth::AuthPayload;

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::meta;

/// Writes an `AuthPayload` containing the credentials for the
/// current session and writes it to `crate::meta::SESSION_PATH`
/// as json.
///
/// # Errors
///
/// Returns `Err` if `fs::try_exists` fails.
/// Returns `Err` if `fs::create_dir_all` fails.
/// Returns `Err` if `File::create` fails.
/// Returns `Err` if `serde_json::to_string_pretty` fails.
/// Returns `Err` if `File::write_all` fails.
///
pub async fn set_session(payload: &AuthPayload) -> anyhow::Result<(), anyhow::Error> {
    let path = PathBuf::from(meta::SESSION_PATH);

    if let Some(parent) = path.parent()
        && !fs::try_exists(meta::SESSION_PATH).await?
    {
        fs::create_dir_all(parent).await?;
    }

    let mut file = File::create(path).await?;

    let payload_json = serde_json::to_string_pretty(payload)?;

    file.write_all(payload_json.as_bytes()).await?;

    Ok(())
}
