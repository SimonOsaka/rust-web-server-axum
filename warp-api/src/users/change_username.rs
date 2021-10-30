use domain::UsersManager;
use serde::Deserialize;
use validator::Validate;
use warp::hyper::StatusCode;

use crate::errors::ChangeUsernameError;
use crate::response::ErrorResponse;
use crate::routes::AuthUser;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangeUsernameForm {
    #[validate(length(min = 2, max = 20, message = "new username length(2-20)"))]
    new_username: String,
}

pub async fn change_username(
    AuthUser(auth_user): AuthUser,
    change_username_form: ChangeUsernameForm,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;

    if manager
        .get_user_by_username(change_username_form.new_username.clone())
        .await
        .is_ok()
    {
        return Err(ErrorResponse::from(ChangeUsernameError::UsernameExist));
    }

    manager
        .change_username(auth_user.get_name(), change_username_form.new_username)
        .await?;

    Ok(StatusCode::OK)
}
