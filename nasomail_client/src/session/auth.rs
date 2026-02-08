//! A utility for storing the user's credentials.

use tokio::{
    fs::{self, File},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::{
    meta,
    session::connection::{self, ConnectionIoError, ConnectionTestError, ConnectionTestResult},
};
use nasomail_shared::{
    api,
    payload::{
        BoolPayload,
        auth::{AuthPayload, PassOnlyAuthPayload},
    },
    query::user::UserQuery,
};

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

/// A custom error type for credentials tests.
#[derive(Debug, thiserror::Error)]
pub enum CredentialsTestError {
    #[error("failed to read saved connection: {0}")]
    ConnectionIoError(ConnectionIoError),

    #[error("failed to read saved credentials: {0}")]
    CredentialsIoError(CredentialsIoError),

    #[error("server responded with a bad status: {0}")]
    BadStatus(reqwest::Error),

    #[error("server responded with a bad json body: {0}")]
    BadResponse(reqwest::Error),

    #[error("connection test failed: {0}")]
    ConnectionTestError(ConnectionTestError),

    #[error("could not reach the server: {0}")]
    ConnectionFailure(reqwest::Error),
}

/// An enum of results for client connection tests.
#[derive(Debug, PartialEq, Eq)]
pub enum CredentialsTestResult {
    Success,
    AuthFailure,
    BadConnection(ConnectionTestResult),
    NoCredentials,
    NoConnection,
}

/// Writes an `AuthPayload` containing
/// the user's credentials as JSON.
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
    let path = meta::credentials_path();

    if let Some(parent) = path.parent()
        && !fs::try_exists(&parent)
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

/// Reads the user's credentials as JSON
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
    let path = meta::credentials_path();

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

/// Removes saved credentials if there are any.
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
    let path = meta::credentials_path();

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

/// Checks if there are any saved credentials.
///
/// Returns `Ok(true)`  if there are saved credentials.
/// Returns `Ok(false)` if there are no saved credentials.
///
/// # Errors:
///
/// Returns `Err(DirError)` if `fs::try_exists` fails.
///
pub async fn has_credentials() -> anyhow::Result<bool, CredentialsIoError> {
    let path = meta::credentials_path();

    Ok(fs::try_exists(path)
        .await
        .map_err(|e| CredentialsIoError::DirError(e))?)
}

/// Checks if the current saved credentials are valid
/// on the currently connected server.
///
/// Returns `Ok(CredentialsTestResult)` unless any unexpected errors occur.
/// Returns `Err(CredentialsTestError)` if the test failed to be performed due to an unexpected error.
///
pub async fn try_credentials() -> anyhow::Result<CredentialsTestResult, CredentialsTestError> {
    let result = connection::try_connection()
        .await
        .map_err(CredentialsTestError::ConnectionTestError)?;

    if result != ConnectionTestResult::Success {
        return Ok(CredentialsTestResult::BadConnection(result));
    }

    let Some(connection) = connection::get_connection()
        .await
        .map_err(CredentialsTestError::ConnectionIoError)?
    else {
        return Ok(CredentialsTestResult::NoConnection);
    };

    let Some(credentials) = get_credentials()
        .await
        .map_err(CredentialsTestError::CredentialsIoError)?
    else {
        return Ok(CredentialsTestResult::NoCredentials);
    };

    let query = UserQuery::ByName {
        name: credentials.username,
    };

    let result = reqwest::Client::new()
        .post(format!(
            "http://{}{}",
            connection,
            api::api_users_auth_absolute()
        ))
        .query(&query)
        .json(&PassOnlyAuthPayload {
            passphrase: credentials.passphrase,
        })
        .send()
        .await
        .map_err(CredentialsTestError::ConnectionFailure)?;

    let result = result
        .error_for_status()
        .map_err(CredentialsTestError::BadStatus)?;

    let payload = result
        .json::<BoolPayload>()
        .await
        .map_err(CredentialsTestError::BadResponse)?;

    Ok(if payload.result {
        CredentialsTestResult::Success
    } else {
        CredentialsTestResult::AuthFailure
    })
}
