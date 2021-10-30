use domain::RegistryUsers;
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::RegistryError;
use crate::response::ErrorResponse;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct RegistryForm {
    #[validate(length(min = 2, max = 20, message = "username length(2-20)"))]
    username: String,
    #[validate(length(min = 8, max = 32, message = "password length(8-32)"))]
    password: String,
}

#[derive(Serialize)]
struct RegistryResponse {
    username: String,
}

pub async fn registry(
    registry_form: RegistryForm,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;
    if manager
        .get_user_by_username(registry_form.username.clone())
        .await
        .is_ok()
    {
        return Err(ErrorResponse::from(RegistryError::UserExist));
    }

    let roles = vec!["user".to_string()];

    let user = RegistryUsers {
        username: registry_form.username,
        password: registry_form.password,
        roles,
    };

    let user = manager.add_user(user).await?;

    Ok(warp::reply::json(&RegistryResponse {
        username: user.username,
    }))
}
