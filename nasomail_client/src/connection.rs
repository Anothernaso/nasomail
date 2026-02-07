//! This module manages connection between
//! the client and the server, and it also provides
//! functions to check the client's current connection status.

use reqwest::StatusCode;
use tokio::{
    fs::{self, File},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::meta;
use nasomail_shared::api;

/// A custom error type for I/O-related
/// errors in connection management.
#[derive(Debug, thiserror::Error)]
pub enum ConnectionIoError {
    #[error("failed to create connection directories: {0}")]
    DirError(io::Error),

    #[error("failed to create/open/remove connection file: {0}")]
    FileError(io::Error),

    #[error("failed to read/write connection file: {0}")]
    RwError(io::Error),
}

/// A custom error type for connection tests.
#[derive(Debug, thiserror::Error)]
pub enum ConnectionTestError {
    #[error("failed to read saved connection: {0}")]
    IoError(ConnectionIoError),
}

/// Writes a `&str` representing the client's current
/// connection, as plain text.
///
/// # Errors
///
/// Returns `Err(DirError)`  if `fs::try_exists` fails.
/// Returns `Err(DirError)`  if `fs::create_dir_all` fails.
/// Returns `Err(FileError)` if `File::create` fails.
/// Returns `Err(RwError)`   if `File::write_all` fails.
///
pub async fn set_connection(connection: &str) -> anyhow::Result<(), ConnectionIoError> {
    let path = meta::connection_path();

    if let Some(parent) = path.parent()
        && !fs::try_exists(&parent)
            .await
            .map_err(|e| ConnectionIoError::DirError(e))?
    {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| ConnectionIoError::DirError(e))?;
    }

    let mut file = File::create(path)
        .await
        .map_err(|e| ConnectionIoError::FileError(e))?;

    file.write_all(connection.trim().as_bytes())
        .await
        .map_err(|e| ConnectionIoError::RwError(e))?;

    Ok(())
}

/// Reads the client's current connection as plain text.
///
/// Returns `Ok(Some(String))` if there is a saved connection.
/// Returns `Ok(None)`         if there is no saved connection.
///
/// # Errors
///
/// Returns `Err(DirError)`  if `fs::try_exists` fails.
/// Returns `Err(FileError)` if `File::open` fails.
/// Returns `Err(RwError)`   if `File::read_to_string` fails.
///
pub async fn get_connection() -> anyhow::Result<Option<String>, ConnectionIoError> {
    let path = meta::connection_path();

    if !has_connection().await? {
        return Ok(None);
    }

    let mut file = File::open(path)
        .await
        .map_err(|e| ConnectionIoError::FileError(e))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .await
        .map_err(|e| ConnectionIoError::RwError(e))?;

    Ok(Some(buf.trim().to_owned()))
}

/// Removes the current saved connection if there is one.
///
/// Returns `Ok(true)`  if the saved connection was removed successfully.
/// Returns `Ok(false)` if there is no saved connection.
///
/// # Errors
///
/// Returns `Err(DirError)`  if `fs::try_exists` fails.
/// Returns `Err(FileError)` if `fs::remove_file` fails.
///
pub async fn remove_connection() -> anyhow::Result<bool, ConnectionIoError> {
    let path = meta::connection_path();

    if !has_connection().await? {
        return Ok(false);
    }

    fs::remove_file(path)
        .await
        .map_err(|e| ConnectionIoError::FileError(e))?;

    Ok(true)
}

/// Checks if there is currently a saved connection.
///
/// Returns `Ok(true)`  if there is a saved connection.
/// Returns `Ok(false)` if there is not a saved connection.
///
/// # Errors:
///
/// Returns `Err(DirError)` if `fs::try_exists` fails.
///
pub async fn has_connection() -> anyhow::Result<bool, ConnectionIoError> {
    let path = meta::connection_path();

    Ok(fs::try_exists(path)
        .await
        .map_err(|e| ConnectionIoError::DirError(e))?)
}

/// Checks if the current saved connection is reachable.
///
/// Returns `Ok`  if the connection could be reached.
/// Returns `Err` if the connection could not be reached.
///
pub async fn test_connection() -> anyhow::Result<bool, ConnectionTestError> {
    let connection = if let Some(connection) = get_connection()
        .await
        .map_err(|e| ConnectionTestError::IoError(e))?
    {
        connection
    } else {
        return Ok(false);
    };

    Ok(
        match reqwest::get(format!("http://{}{}", connection, api::CTEST)).await {
            Ok(res) => res.status() == StatusCode::OK,
            Err(_) => false,
        },
    )
}
