use domain::{GetUserError, UsersManager};
use serde::Serialize;

use crate::response::ErrorResponse;
use crate::routes::AuthUser;
use crate::AppState;

#[derive(Serialize)]
pub struct MeResponse {
    pub username: String,
    pub roles: Vec<String>,
}

pub async fn me(
    AuthUser(auth_user): AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;

    match manager.get_user_by_username(auth_user.get_name()).await {
        Ok(user) => Ok(warp::reply::json(&MeResponse {
            username: user.username,
            roles: user.roles,
        })),
        Err(_) => Err(ErrorResponse::from(GetUserError::NotFound {
            username: auth_user.get_name(),
        })),
    }
}
