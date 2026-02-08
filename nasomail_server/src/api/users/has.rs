use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response,
    routing::get,
};

use nasomail_shared::api;
use nasomail_shared::payload::BoolPayload;
use nasomail_shared::query::user::UserQuery;
use tracing::instrument;

use crate::app::AppContextGuard;

pub trait RouterApiHas {
    /// Registers routes for
    /// user related APIs
    fn with_api_has(self) -> Self;
}

impl RouterApiHas for Router<AppContextGuard> {
    fn with_api_has(self) -> Self {
        self.route(api::API_USERS_HAS, get(handle))
    }
}

#[instrument(skip(app, q))]
async fn handle(
    State(app): State<AppContextGuard>,
    Query(q): Query<UserQuery>,
) -> response::Result<Json<BoolPayload>, StatusCode> {
    let ctx = app.ctx().await;
    let pool = ctx.pool().await;

    let exists: bool = match q {
        UserQuery::ById { id } => {
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id = ?)")
                .bind(id)
                .fetch_one(&*pool)
                .await
        }
        UserQuery::ByName { name } => {
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE name = ?)")
                .bind(name)
                .fetch_one(&*pool)
                .await
        }
    }
    .map_err(|e| {
        tracing::error!(err = ?e, "internal server error");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(BoolPayload { result: exists }))
}
