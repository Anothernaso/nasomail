//! A utility for reading and writing the
//! user's current session.

use std::path::PathBuf;

use nasomail_shared::auth::AuthPayload;

use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
};

use crate::meta;

/// A custom error type for I/O-related
/// errors in session management.
#[derive(Debug, thiserror::Error)]
pub enum SessionIoError {
    #[error("failed to create session directories: {0}")]
    MkdirError(io::Error),

    #[error("failed to create/open session file: {0}")]
    FileError(io::Error),

    #[error("failed to serialize `payload`: {0}")]
    SerError(serde_json::Error),

    #[error("failed to read/write session file: {0}")]
    RwError(io::Error),
}

/// Writes an `AuthPayload` containing the credentials for the
/// current session and writes it to `crate::meta::SESSION_PATH`
/// as json.
///
/// # Errors
///
/// Returns `MkdirError` if `fs::try_exists` fails.
/// Returns `MkdirError` if `fs::create_dir_all` fails.
/// Returns `FileError` if `File::create` fails.
/// Returns `SerError` if `serde_json::to_string_pretty` fails.
/// Returns `RwError` if `File::write_all` fails.
///
pub async fn set_session(payload: &AuthPayload) -> anyhow::Result<(), SessionIoError> {
    let path = PathBuf::from(meta::SESSION_PATH);

    if let Some(parent) = path.parent()
        && !fs::try_exists(meta::SESSION_PATH)
            .await
            .map_err(|e| SessionIoError::MkdirError(e))?
    {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| SessionIoError::MkdirError(e))?;
    }

    let mut file = File::create(path)
        .await
        .map_err(|e| SessionIoError::FileError(e))?;

    let payload_json =
        serde_json::to_string_pretty(payload).map_err(|e| SessionIoError::SerError(e))?;

    file.write_all(payload_json.as_bytes())
        .await
        .map_err(|e| SessionIoError::RwError(e))?;

    Ok(())
}
