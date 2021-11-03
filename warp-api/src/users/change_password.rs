use domain::UsersManager;
use serde::Deserialize;
use validator::Validate;
use warp::hyper::StatusCode;

use crate::response::ErrorResponse;
use crate::routes::AuthUser;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangePasswordForm {
    #[validate(length(min = 8, max = 32, message = "old password length(8-32)"))]
    old_password: String,
    #[validate(length(min = 8, max = 32, message = "new password length(8-32)"))]
    new_password: String,
}

pub async fn change_password(
    AuthUser(auth_user): AuthUser,
    change_password_form: ChangePasswordForm,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;

    let user = manager
        .get_user(auth_user.get_name(), change_password_form.old_password)
        .await?;

    user.change_password(change_password_form.new_password, manager)
        .await?;

    Ok(StatusCode::OK)
}
