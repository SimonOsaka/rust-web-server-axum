use domain::UsersManager;
use serde::Deserialize;
use validator::Validate;
use warp::hyper::StatusCode;
use warp::{reject::custom, Filter, Rejection};

use crate::errors::{ChangeUsernameError, ValidateError};
use crate::response::ErrorResponse;
use crate::routes::AuthUser;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct ChangeUsernameForm {
    #[validate(length(min = 2, max = 20, message = "new username length(2-20)"))]
    new_username: String,
}

pub fn with_json_validate(
) -> impl Filter<Extract = (ChangeUsernameForm,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json().and_then(
        |val: ChangeUsernameForm| async move {
            match val.validate() {
                Ok(_) => Ok(val),
                Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
            }
        },
    ))
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
