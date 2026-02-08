mod auth;
mod has;

use crate::{
    api::users::{auth::RouterApiUsersAuth, has::RouterApiUsersHas},
    app::AppContextGuard,
};
use axum::Router;

use nasomail_shared::api;

pub trait RouterApiUsers {
    /// Registers routes for
    /// user related APIs
    fn with_api_users(self) -> Self;
}

impl RouterApiUsers for Router<AppContextGuard> {
    fn with_api_users(self) -> Self {
        self.nest(
            api::API_USERS,
            Router::new().with_api_users_has().with_api_users_auth(),
        )
    }
}
