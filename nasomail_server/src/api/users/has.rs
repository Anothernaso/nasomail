use crate::app::AppContextGuard;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use nasomail_shared::api;

pub trait RouterApiHas {
    /// Registers routes for
    /// user related APIs
    fn with_api_has(self) -> Self;
}

impl RouterApiHas for Router<AppContextGuard> {
    fn with_api_has(self) -> Self {
        self.route(
            api::API_USERS_HAS,
            get(|State(_app): State<AppContextGuard>| async {}),
        )
    }
}
