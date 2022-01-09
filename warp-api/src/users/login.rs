use auth::encode_token;
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::LoginError;
use crate::response::ErrorResponse;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct LoginForm {
    #[validate(length(min = 2, max = 20, code = "login-valid-username"))]
    username: String,
    #[validate(length(min = 8, max = 32, code="login-valid-password"))]
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[tracing::instrument(skip(state))]
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
