pub mod has;

use crate::app::AppContextGuard;
use axum::Router;

use has::RouterApiHas;
use nasomail_shared::api;

pub trait RouterApiUsers {
    /// Registers routes for
    /// user related APIs
    fn with_api_users(self) -> Self;
}

impl RouterApiUsers for Router<AppContextGuard> {
    fn with_api_users(self) -> Self {
        self.nest(api::API_USERS, Router::new().with_api_has())
    }
}
