//! This module contains payloads
//! related to the creation of user accounts.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RegisterResultPayload {
    Success { user_id: i64 },
    BadCredentials { details: String },
}
