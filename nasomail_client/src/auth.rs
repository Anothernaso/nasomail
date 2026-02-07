//! A utility for storing the user's credentials.

use tokio::{
    fs::{self, File},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::meta;
use nasomail_shared::auth::AuthPayload;

/// A custom error type for I/O-related
/// errors in credentials management.
#[derive(Debug, thiserror::Error)]
pub enum CredentialsIoError {
    #[error("failed to create credentials directories: {0}")]
    DirError(io::Error),

    #[error("failed to create/open/remove credentials file: {0}")]
    FileError(io::Error),

    #[error("failed to serialize `payload`: {0}")]
    SerError(serde_json::Error),

    #[error("failed to read/write credentials file: {0}")]
    RwError(io::Error),
}

/// Writes an `AuthPayload` containing the user's
/// credentials to `crate::meta::CREDENTIALS_PATH` as JSON.
///
/// # Errors
///
/// Returns `Err(DirError)`  if `fs::try_exists` fails.
/// Returns `Err(DirError)`  if `fs::create_dir_all` fails.
/// Returns `Err(FileError)` if `File::create` fails.
/// Returns `Err(SerError)`  if `serde_json::to_string_pretty` fails.
/// Returns `Err(RwError)`   if `File::write_all` fails.
///
pub async fn set_credentials(payload: &AuthPayload) -> anyhow::Result<(), CredentialsIoError> {
    let path = meta::credentials_path().expect("failed to get credentials path");

    if let Some(parent) = path.parent()
        && !fs::try_exists(&path)
            .await
            .map_err(|e| CredentialsIoError::DirError(e))?
    {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| CredentialsIoError::DirError(e))?;
    }

    let mut file = File::create(path)
        .await
        .map_err(|e| CredentialsIoError::FileError(e))?;

    let payload_json =
        serde_json::to_string_pretty(payload).map_err(|e| CredentialsIoError::SerError(e))?;

    file.write_all(payload_json.as_bytes())
        .await
        .map_err(|e| CredentialsIoError::RwError(e))?;

    Ok(())
}

/// Reads the user's credentials from `crate::meta::CREDENTIALS_PATH` as JSON
/// and returns it as an `AuthPayload`.
///
/// Returns `Ok(Some(AuthPayload))` if there are saved credentials.
/// Returns `Ok(None)`              if there are no saved credentials.
///
/// # Errors
///
/// Returns `Err(DirError)`  if `fs::try_exists` fails.
/// Returns `Err(FileError)` if `File::open` fails.
/// Returns `Err(RwError)`   if `File::read_to_string` fails.
/// Returns `Err(SerError)`  if `serde_json::from_str` fails.
///
pub async fn get_credentials() -> anyhow::Result<Option<AuthPayload>, CredentialsIoError> {
    let path = meta::credentials_path().expect("failed to get credentials path");

    if !fs::try_exists(&path)
        .await
        .map_err(|e| CredentialsIoError::DirError(e))?
    {
        return Ok(None);
    }

    let mut file = File::open(path)
        .await
        .map_err(|e| CredentialsIoError::FileError(e))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .await
        .map_err(|e| CredentialsIoError::RwError(e))?;

    let payload =
        serde_json::from_str::<AuthPayload>(&buf).map_err(|e| CredentialsIoError::SerError(e))?;

    Ok(Some(payload))
}

/// Removes saved credentials at `crate::meta::CREDENTIALS_PATH` if there are any.
///
/// Returns `Ok(true)`  if the saved credentials were removed successfully.
/// Returns `Ok(false)` if there were no saved credentials.
///
/// # Errors
///
/// Returns `Err(DirError)`  if `fs::try_exists` fails.
/// Returns `Err(FileError)` if `fs::remove_file` fails.
///
pub async fn remove_credentials() -> anyhow::Result<bool, CredentialsIoError> {
    let path = meta::credentials_path().expect("failed to get credentials path");

    if !fs::try_exists(&path)
        .await
        .map_err(|e| CredentialsIoError::DirError(e))?
    {
        return Ok(false);
    }

    fs::remove_file(path)
        .await
        .map_err(|e| CredentialsIoError::FileError(e))?;

    Ok(true)
}

// TODO: Add function for checking if credentials are valid
