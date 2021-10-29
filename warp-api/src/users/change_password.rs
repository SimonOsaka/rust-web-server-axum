use domain::UsersManager;
use serde::Deserialize;
use validator::Validate;
use warp::hyper::StatusCode;
use warp::{reject::custom, Filter, Rejection};

use crate::errors::ValidateError;
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

pub fn with_json_validate(
) -> impl Filter<Extract = (ChangePasswordForm,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json().and_then(
        |val: ChangePasswordForm| async move {
            match val.validate() {
                Ok(_) => Ok(val),
                Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
            }
        },
    ))
}

pub async fn change_password(
    AuthUser(auth_user): AuthUser,
    change_password_form: ChangePasswordForm,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;

    manager
        .get_user(auth_user.get_name(), change_password_form.old_password)
        .await?;

    manager
        .change_password(auth_user.get_name(), change_password_form.new_password)
        .await?;

    Ok(StatusCode::OK)
}
