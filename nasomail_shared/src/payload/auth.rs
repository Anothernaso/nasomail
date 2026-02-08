use serde::{Deserialize, Serialize};

/// A simple struct that contains everything
/// necessary to authenticate a user.
#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub username: String, // This explicitly be the name of the user (i.e, username)
    pub passphrase: String,
}

/// A simple struct that contains the
/// passphrase of a user.
///
/// For use when the user account is
/// specified by a query parameter.
///
#[derive(Serialize, Deserialize)]
pub struct PassOnlyAuthPayload {
    pub passphrase: String,
}
