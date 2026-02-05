use axum::Router;
use axum::extract::State;
use axum::routing::get;
use reqwest::StatusCode;
use tracing::{info, instrument, warn};

use crate::app::AppContextGuard;
use nasomail_shared::api;

pub trait RouterCtest {
    fn with_ctest(self) -> Self;
}

impl RouterCtest for Router<AppContextGuard> {
    #[instrument]
    fn with_ctest(self) -> Self {
        self.route(
            api::TEST_CODE,
            get(|State(app): State<AppContextGuard>| async move {
                app.ctx().await.test_code().await.clone()
            }),
        )
    }
}

#[instrument(skip(app))]
pub async fn connection_test(app: AppContextGuard) {
    let ctx = app.ctx().await;
    let cfg = ctx.cfg().await;
    let pub_addr = cfg.pub_addr().await;

    info!(pub_addr = %pub_addr, "performing");

    let response = reqwest::get(format!("http://{}{}", pub_addr, api::TEST_CODE)).await;
    if let Err(e) = response {
        warn!(err = ?e, "failed: could not reach server");
        return;
    }
    let response = response.unwrap();

    let status = response.status();
    if status != StatusCode::OK {
        warn!(status = %status, expected = %StatusCode::OK, "failed: status mismatch");
        return;
    }

    let text = response.text().await;
    if let Err(e) = &text {
        warn!(err = ?e, "failed: could not read body");
    }
    let test_code = text.unwrap();
    let test_code = test_code.trim();

    let expected = ctx.test_code().await;

    if test_code != *expected {
        warn!(
            test_code = test_code,
            expected = *expected,
            "failed: test code mismatch"
        );
        return;
    }

    info!(test_code = test_code, expected = *expected, "succeeded");
}
