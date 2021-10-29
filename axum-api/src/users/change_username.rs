use axum::extract::Extension;
use domain::UsersManager;
use hyper::StatusCode;
use serde::Deserialize;
use validator::Validate;

use crate::{
    app_error::ChangeUsernameError,
    app_request::{AuthUser, ValidatedJson},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangeUsernameForm {
    #[validate(length(min = 2, max = 20, message = "new username length(2-20)"))]
    new_username: String,
}

pub async fn change_username(
    AuthUser(auth_user): AuthUser,
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

    manager
        .change_username(auth_user.get_name(), change_username_form.new_username)
        .await?;

    Ok(StatusCode::OK)
}
