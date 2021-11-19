use auth::encode_token;
use axum::{extract::Extension, Json};
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{app_error::LoginError, app_request::ValidatedJson, app_response::AppError, AppState};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct LoginForm {
    #[validate(length(min = 2, max = 20, message = "username length(2-20)"))]
    username: String,
    #[validate(length(min = 8, max = 32, message = "password length(8-32)"))]
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[tracing::instrument(skip(state))]
pub async fn login(
    ValidatedJson(login_form): ValidatedJson<LoginForm>,
    Extension(state): Extension<AppState>,
) -> Result<Json<LoginResponse>, AppError> {
    let manager = &state.users_manager;

    let tuple = manager
        .verify_user(login_form.username.clone(), login_form.password)
        .await?;

    let (pass, user) = tuple;

    match pass {
        true => {
            let token = encode_token(user.id, login_form.username, user.roles);
            Ok(LoginResponse { token }.into())
        }
        false => Err(AppError::from(LoginError::WrongPassword)),
    }
}
