use axum::{Json, Router, http::StatusCode, response, routing::get};
use tracing::instrument;

use nasomail_shared::api;

use nasomail_shared::payload::{auth::AuthPayload, register::RegisterResultPayload};

use crate::app::AppContextGuard;

pub trait RouterApiUsersRegister {
    /// Registers the `/api/users/has` endpoint
    /// which checks whether or not the database
    /// has the specified user.
    fn with_api_users_register(self) -> Self;
}

impl RouterApiUsersRegister for Router<AppContextGuard> {
    fn with_api_users_register(self) -> Self {
        self.route(api::API_USERS_REGISTER, get(handle))
    }
}

#[axum::debug_handler]
#[instrument(skip(payload))]
async fn handle(
    Json(payload): Json<AuthPayload>,
) -> response::Result<Json<RegisterResultPayload>, StatusCode> {
    Ok(Json(RegisterResultPayload::Success { user_id: 0 })) // TODO: Implement database query
}
