use axum::extract::Extension;
use domain::UsersManager;
use hyper::StatusCode;
use serde::Deserialize;
use validator::Validate;

use crate::{
    app_error::ChangeUsernameError,
    app_request::{JwtAuth, ValidatedJson},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangeUsernameForm {
    #[validate(length(min = 2, max = 20, code = "user-change-username-valid-new_username"))]
    new_username: String,
}

#[tracing::instrument(skip(auth_user, state))]
pub async fn change_username(
    JwtAuth(auth_user): JwtAuth,
    ValidatedJson(change_username_form): ValidatedJson<ChangeUsernameForm>,
    Extension(state): Extension<AppState>,
) -> Result<StatusCode, AppError> {
    let manager = &state.users_manager;

    if manager
        .get_user_by_username(change_username_form.new_username.clone())
        .await
        .is_ok()
    {
        return Err(AppError::from(ChangeUsernameError::UsernameExist));
    }

    let user = manager.get_user_by_username(auth_user.get_name()).await?;

    user.change_username(change_username_form.new_username, manager)
        .await?;

    Ok(StatusCode::OK)
}
