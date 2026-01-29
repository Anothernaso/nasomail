use axum::Router;
use axum::extract::State;
use axum::routing::get;
use tracing::{info, instrument, warn};

use crate::app::AppContextGuardPtr;

pub trait RouterCtest {
    fn with_ctest(self) -> Self;
}

impl RouterCtest for Router<AppContextGuardPtr> {
    #[instrument]
    fn with_ctest(self) -> Self {
        self.route(
            "/ctest/test_code",
            get(|State(app): State<AppContextGuardPtr>| async move {
                app.ctx().await.test_code().await.clone()
            }),
        )
    }
}

#[instrument(skip(app))]
pub async fn connection_test(app: AppContextGuardPtr) {
    let ctx = app.ctx().await;
    let cfg = ctx.cfg().await;
    let pub_addr = cfg.pub_addr().await;

    info!(pub_addr = %pub_addr, "performing");

    let response = reqwest::get(format!("http://{pub_addr}/ctest/test_code")).await;
    if let Err(e) = response {
        warn!(err = ?e, "failed: could not reach server");
        return;
    }
    let response = response.unwrap();

    let status = response.status();
    if status != 200 {
        warn!(status = %status, expected = 200, "failed: wrong status");
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
