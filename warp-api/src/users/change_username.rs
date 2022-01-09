use domain::UsersManager;
use serde::Deserialize;
use validator::Validate;
use warp::hyper::StatusCode;

use crate::errors::ChangeUsernameError;
use crate::request::AuthUser;
use crate::response::ErrorResponse;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangeUsernameForm {
    #[validate(length(min = 2, max = 20, code = "user-change-username-valid-new_username"))]
    new_username: String,
}

#[tracing::instrument(skip(auth_user, state))]
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

    let user = manager.get_user_by_username(auth_user.get_name()).await?;

    user.change_username(change_username_form.new_username, manager)
        .await?;

    Ok(StatusCode::OK)
}
