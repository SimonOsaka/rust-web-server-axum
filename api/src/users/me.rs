use axum::{extract::Extension, Json};
use domain::{GetUserError, UsersManager};
use macros::router;
use serde::Serialize;

use crate::{app_request::JwtAuth, app_response::AppError, AppState};

#[derive(Serialize)]
pub struct MeResponse {
    pub username: String,
    pub roles: Vec<String>,
}

#[tracing::instrument(skip(auth_user, state))]
#[router(path="/api/users/me")]
pub async fn me(
    JwtAuth(auth_user): JwtAuth,
    Extension(state): Extension<AppState>,
) -> Result<Json<MeResponse>, AppError> {
    let manager = &state.users_manager;

    match manager.get_user_by_username(auth_user.get_name()).await {
        Ok(user) => Ok(MeResponse {
            username: user.username,
            roles: user.roles,
        }
        .into()),
        Err(_) => Err(AppError::from(GetUserError::NotFound {
            username: auth_user.get_name(),
        })),
    }
}
