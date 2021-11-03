use axum::extract::Extension;
use domain::UsersManager;
use hyper::StatusCode;
use serde::Deserialize;
use validator::Validate;

use crate::{
    app_request::{AuthUser, ValidatedJson},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangePasswordForm {
    #[validate(length(min = 8, max = 32, message = "old password length(8-32)"))]
    old_password: String,
    #[validate(length(min = 8, max = 32, message = "new password length(8-32)"))]
    new_password: String,
}

pub async fn change_password(
    AuthUser(auth_user): AuthUser,
    ValidatedJson(change_password_form): ValidatedJson<ChangePasswordForm>,
    Extension(state): Extension<AppState>,
) -> Result<StatusCode, AppError> {
    let manager = &state.users_manager;

    let user = manager
        .get_user(auth_user.get_name(), change_password_form.old_password)
        .await?;

    user.change_password(change_password_form.new_password, manager)
        .await?;

    Ok(StatusCode::OK)
}
