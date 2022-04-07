use axum::{extract::Extension, Json};
use domain::UsersManager;
use macros::router;
use tracing::debug;

use crate::{
    app_request::JwtAuth, app_response::AppError, response::MyAdventuresResponse, AppState,
};

#[tracing::instrument(skip(state))]
#[router(path = "/api/adventures/my")]
pub async fn my_list_adventures(
    JwtAuth(user): JwtAuth,
    Extension(state): Extension<AppState>,
) -> Result<Json<MyAdventuresResponse>, AppError> {
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(user.get_name()).await?;
    let adventures = user.find_adventures(&state.adventures_manager).await?;
    let response = MyAdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response.into())
}
