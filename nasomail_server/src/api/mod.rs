//! This module contains all routing for
//! the REST API routing in `nasomail_server`.

use axum::Router;

pub mod ctest;
mod users;

use crate::api::ctest::RouterApiCtest;
use crate::api::users::RouterApiUsers;
use crate::app::AppContextGuard;

use nasomail_shared::api;

pub trait RouterApi {
    /// Registers all the routing for the
    /// entire REST API in `nasomail_server`.
    fn with_api(self) -> Self;
}

impl RouterApi for Router<AppContextGuard> {
    fn with_api(self) -> Self {
        self.nest(api::API, Router::new().with_api_ctest().with_api_users())
    }
}
