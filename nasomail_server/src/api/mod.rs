//! This module contains all routing for
//! the REST API routing in `nasomail_server`.

use axum::Router;

pub mod ctest;
pub mod users;

use crate::api::ctest::RouterApiCtest;
use crate::api::users::RouterApiUsers;
use crate::app::AppContextGuard;

use nasomail_shared::api;

pub trait RouterApiRoot {
    /// Registers all the routing for the
    /// entire REST API in `nasomail_server`.
    fn with_api_root(self) -> Self;
}

impl RouterApiRoot for Router<AppContextGuard> {
    fn with_api_root(self) -> Self {
        self.nest(
            api::API_ROOT,
            Router::new().with_api_ctest().with_api_users(),
        )
    }
}
