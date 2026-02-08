use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response,
    routing::post,
};

use tracing::instrument;

use nasomail_shared::payload::BoolPayload;
use nasomail_shared::query::user::UserQuery;
use nasomail_shared::{api, payload::auth::PassOnlyAuthPayload};

use crate::app::AppContextGuard;

pub trait RouterApiUsersAuth {
    /// Registers the `/api/users/has` endpoint
    /// which checks whether or not the database
    /// has the specified user.
    fn with_api_users_auth(self) -> Self;
}

impl RouterApiUsersAuth for Router<AppContextGuard> {
    fn with_api_users_auth(self) -> Self {
        self.route(api::API_USERS_AUTH, post(handle))
    }
}

/// Checks if the `passphrase` of the provided `PassOnlyAuthPayload`
/// matches the `passphrase` of the user in the `users` table of the database
/// specified by the `id` or `name` fields of the provided `UserQuery`.
#[instrument(skip(app, query, payload))]
async fn handle(
    State(app): State<AppContextGuard>,
    Query(query): Query<UserQuery>,
    Json(payload): Json<PassOnlyAuthPayload>,
) -> response::Result<Json<BoolPayload>, StatusCode> {
    let ctx = app.ctx().await;
    let pool = ctx.pool().await;

    let result: Option<i64> = match query {
        UserQuery::ById { id } => {
            sqlx::query_scalar(
                "SELECT CASE WHEN passphrase = ? THEN 1 ELSE 0 END FROM users WHERE id = ?",
            )
            .bind(payload.passphrase)
            .bind(id)
            .fetch_optional(&*pool)
            .await
        }
        UserQuery::ByName { name } => {
            sqlx::query_scalar(
                "SELECT CASE WHEN passphrase = ? THEN 1 ELSE 0 END FROM users WHERE name = ?",
            )
            .bind(payload.passphrase)
            .bind(name)
            .fetch_optional(&*pool)
            .await
        }
    }
    .map_err(|e| {
        tracing::info!(err = ?e, "internal server error");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(BoolPayload {
        result: result.unwrap_or(0) != 0,
    }))
}
