//! A utility for reading and writing the
//! user's current session.

use std::path::PathBuf;

use nasomail_shared::auth::AuthPayload;

use tokio::{
    fs::{self, File},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::meta;

/// A custom error type for I/O-related
/// errors in session management.
#[derive(Debug, thiserror::Error)]
pub enum SessionIoError {
    #[error("failed to create session directories: {0}")]
    DirError(io::Error),

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
/// Returns `Err(DirError)` if `fs::try_exists` fails.
/// Returns `Err(DirError)` if `fs::create_dir_all` fails.
/// Returns `Err(FileError)` if `File::create` fails.
/// Returns `Err(SerError)` if `serde_json::to_string_pretty` fails.
/// Returns `Err(RwError)` if `File::write_all` fails.
///
pub async fn set_session(payload: &AuthPayload) -> anyhow::Result<(), SessionIoError> {
    let path = PathBuf::from(meta::SESSION_PATH);

    if let Some(parent) = path.parent()
        && !fs::try_exists(meta::SESSION_PATH)
            .await
            .map_err(|e| SessionIoError::DirError(e))?
    {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| SessionIoError::DirError(e))?;
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

/// Reads the current session file and returns
/// information about the current session as an `AuthPayload`.
///
/// Returns `Ok(Some(AuthPayload))` if there are/is (a) saved session/credentials.
/// Returns `Ok(None)` if there are/is no saved session/credentials.
///
/// # Errors
///
/// Returns `Err(DirError)` if `fs::try_exists` fails.
/// Returns `Err(FileError)` if `File::open` fails.
/// Returns `Err(RwError)` if `File::read_to_string` fails.
/// Returns `Err(SerError)` if `serde_json::from_str` fails.
///
pub async fn get_session() -> anyhow::Result<Option<AuthPayload>, SessionIoError> {
    let path = PathBuf::from(meta::SESSION_PATH);

    if !fs::try_exists(&path)
        .await
        .map_err(|e| SessionIoError::DirError(e))?
    {
        return Ok(None);
    }

    let mut file = File::open(path)
        .await
        .map_err(|e| SessionIoError::FileError(e))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .await
        .map_err(|e| SessionIoError::RwError(e))?;

    let payload =
        serde_json::from_str::<AuthPayload>(&buf).map_err(|e| SessionIoError::SerError(e))?;

    Ok(Some(payload))
}
