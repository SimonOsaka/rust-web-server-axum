use auth::encode_token;
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;
use warp::{reject::custom, Filter, Rejection};

use crate::errors::{LoginError, ValidateError};
use crate::response::ErrorResponse;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct LoginForm {
    #[validate(length(min = 2, max = 20, message = "username length(2-20)"))]
    username: String,
    #[validate(length(min = 8, max = 32, message = "password length(8-32)"))]
    password: String,
}

pub fn with_json_validate() -> impl Filter<Extract = (LoginForm,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json().and_then(
        |val: LoginForm| async move {
            match val.validate() {
                Ok(_) => Ok(val),
                Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
            }
        },
    ))
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

pub async fn login(
    login_form: LoginForm,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;

    let tuple = manager
        .verify_user(login_form.username.clone(), login_form.password)
        .await?;

    let (pass, user) = tuple;

    match pass {
        true => {
            let token = encode_token(user.id, login_form.username, user.roles);
            Ok(warp::reply::json(&LoginResponse { token }))
        }
        false => Err(ErrorResponse::from(LoginError::WrongPassword)),
    }
}
