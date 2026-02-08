//! Contains information about the routing
//! for the `nasomail_server` REST API.

pub const API_ROOT: &str = "/api";
pub const API_CTEST: &str = "/ctest";
pub const API_USERS: &str = "/users";
pub const API_USERS_HAS: &str = "/has";

pub fn api_root_absolute() -> String {
    format!("{}", API_ROOT)
}

pub fn api_ctest_absolute() -> String {
    format!("{}{}", api_root_absolute(), API_CTEST)
}

pub fn api_users_absolute() -> String {
    format!("{}{}", api_root_absolute(), API_USERS)
}

pub fn api_users_has_absolute() -> String {
    format!("{}{}", api_users_absolute(), API_USERS_HAS)
}
