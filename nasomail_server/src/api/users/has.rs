use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response,
    routing::get,
};

use tracing::instrument;

use nasomail_shared::api;
use nasomail_shared::payload::BoolPayload;
use nasomail_shared::query::user::UserQuery;

use crate::app::AppContextGuard;

pub trait RouterApiUsersHas {
    /// Registers the `/api/users/has` endpoint
    /// which checks whether or not the database
    /// has the specified user.
    fn with_api_users_has(self) -> Self;
}

impl RouterApiUsersHas for Router<AppContextGuard> {
    fn with_api_users_has(self) -> Self {
        self.route(api::API_USERS_HAS, get(handle))
    }
}

/// This endpoint takes in the `id` or `name` of a user
/// and checks if it exists in the `users` table of the database,
/// then it returns a `BoolPayload` where the `result` field
/// represents whether or not the database has the specified user.
#[axum::debug_handler]
#[instrument(skip(app, query))]
async fn handle(
    State(app): State<AppContextGuard>,
    Query(query): Query<UserQuery>,
) -> response::Result<Json<BoolPayload>, StatusCode> {
    let ctx = app.ctx().await;
    let pool = ctx.pool().await;

    let exists: bool = match query {
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
