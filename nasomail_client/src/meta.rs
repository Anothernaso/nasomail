//! This module contains constants with information about
//! the application such as the location of various files.

use std::{env, path::PathBuf};

/// Gets the path to the credentials file.
///
/// # Panics
///
/// Panics if `std::env::home_dir` returns `None`
///
pub fn credentials_path() -> PathBuf {
    env::home_dir()
        .expect("failed to get home directory")
        .join(format!(
            ".config/{}/credentials.json",
            env!("CARGO_PKG_NAME")
        ))
}

/// Gets the path to the connection file.
///
/// # Panics
///
/// Panics if `std::env::home_dir` returns `None`
///
pub fn connection_path() -> PathBuf {
    env::home_dir()
        .expect("failed to get home directory")
        .join(format!(".config/{}/connection.txt", env!("CARGO_PKG_NAME")))
}
