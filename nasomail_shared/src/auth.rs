use serde::{Deserialize, Serialize};

/// A simple struct that contains everything
/// necessary to authenticate a user.
#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub username: String, // This explicitly be the name of the user (i.e, username)
    pub passphrase: String,
}
