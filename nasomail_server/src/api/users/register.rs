use axum::{Router, routing::get};
use tracing::instrument;

use nasomail_shared::api;

use crate::app::AppContextGuard;

pub trait RouterApiUsersRegister {
    /// Registers the `/api/users/has` endpoint
    /// which checks whether or not the database
    /// has the specified user.
    fn with_api_users_register(self) -> Self;
}

impl RouterApiUsersRegister for Router<AppContextGuard> {
    fn with_api_users_register(self) -> Self {
        self.route(api::API_USERS_REGISTER, get(handle))
    }
}

#[instrument]
async fn handle() {}
