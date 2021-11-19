use axum::{extract::Extension, Json};
use domain::UsersManager;
use tracing::debug;

use crate::{
    app_request::AuthUser, app_response::AppError, response::MyAdventuresResponse, AppState,
};

#[tracing::instrument(skip(state))]
pub async fn my_list_adventures(
    AuthUser(user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<MyAdventuresResponse>, AppError> {
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(user.get_name()).await?;
    let adventures = user.find_adventures(&state.adventures_manager).await?;
    let response = MyAdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response.into())
}
