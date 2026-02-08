//! The root module of all payloads in
//! the `nasomail_server` REST API.
//!
//! Also contains some misc payloads.
//!

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod register;

#[derive(Serialize, Deserialize)]
pub struct BoolPayload {
    pub result: bool,
}
