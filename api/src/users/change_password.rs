use axum::extract::Extension;
use domain::UsersManager;
use hyper::StatusCode;
use serde::Deserialize;
use validator::Validate;

use crate::{
    app_request::{JwtAuth, ValidatedJson},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangePasswordForm {
    #[validate(length(min = 8, max = 32, code = "user-change-password-valid-old_password"))]
    old_password: String,
    #[validate(length(min = 8, max = 32, code = "user-change-password-valid-new_password"))]
    new_password: String,
}

#[tracing::instrument(skip(auth_user, state, change_password_form))]
pub async fn change_password(
    JwtAuth(auth_user): JwtAuth,
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
