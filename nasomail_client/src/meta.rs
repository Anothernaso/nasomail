//! This module contains constants with information about
//! the application such as the location of various files.

use std::{env, path::PathBuf};

/// Gets the path to the credentials file.
pub fn credentials_path() -> Option<PathBuf> {
    Some(env::home_dir()?.join(format!(
        ".config/{}/credentials.json",
        env!("CARGO_PKG_NAME")
    )))
}

/// Gets the path to the connection file.
pub fn connection_path() -> Option<PathBuf> {
    Some(env::home_dir()?.join(format!(".config/{}/connection", env!("CARGO_PKG_NAME"))))
}
