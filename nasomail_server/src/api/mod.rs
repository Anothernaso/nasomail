//! This module contains all routing for
//! the REST API routing in `nasomail_server`.

use axum::Router;

pub mod ctest;

use crate::api::ctest::RouterCtest;
use crate::app::AppContextGuard;

pub trait RouterApi {
    /// Registers all the routing for the
    /// entire REST API in `nasomail_server`.
    fn with_api(self) -> Self;
}

impl RouterApi for Router<AppContextGuard> {
    fn with_api(self) -> Self {
        self.with_ctest()
    }
}
