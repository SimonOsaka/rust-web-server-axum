use crate::{app_request::JwtAuth, app_response::AppError, AppState};
use axum::{extract::State, Json};
use domain::{GetUserError, UsersManager};
use macros::router;
use serde::Serialize;

#[derive(Serialize)]
pub struct MeResponse {
    pub username: String,
    pub roles: Vec<String>,
}

#[tracing::instrument(skip(auth_user, state))]
#[router(path = "/api/users/me")]
async fn me(
    JwtAuth(auth_user): JwtAuth,
    State(state): State<AppState>,
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
