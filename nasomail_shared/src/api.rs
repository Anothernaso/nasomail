//! Contains information about the routing
//! for the `nasomail_server` REST API.

pub const API: &str = "/api";
pub const API_CTEST: &str = "/ctest";

pub const API_USERS: &str = "/users";
pub const API_USERS_HAS: &str = "/has";
pub const API_USERS_AUTH: &str = "/auth";
pub const API_USERS_REGISTER: &str = "/register";

pub fn api_absolute() -> String {
    format!("{}", API)
}

pub fn api_ctest_absolute() -> String {
    format!("{}{}", api_absolute(), API_CTEST)
}

pub fn api_users_absolute() -> String {
    format!("{}{}", api_absolute(), API_USERS)
}

pub fn api_users_has_absolute() -> String {
    format!("{}{}", api_users_absolute(), API_USERS_HAS)
}

pub fn api_users_auth_absolute() -> String {
    format!("{}{}", api_users_absolute(), API_USERS_AUTH)
}

pub fn api_users_register_absolute() -> String {
    format!("{}{}", api_users_absolute(), API_USERS_REGISTER)
}
