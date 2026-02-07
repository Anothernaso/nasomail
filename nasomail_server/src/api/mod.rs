use axum::Router;

pub mod ctest;

use crate::api::ctest::RouterCtest;
use crate::app::AppContextGuard;

pub trait RouterApi {
    fn with_api(self) -> Self;
}

impl RouterApi for Router<AppContextGuard> {
    fn with_api(self) -> Self {
        self.with_ctest()
    }
}
