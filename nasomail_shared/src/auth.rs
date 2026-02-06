use serde::{Deserialize, Serialize};

/// A simple struct that contains everything
/// necessary to authenticate a user.
#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub name: String,
    pub passphrase: String,
}
