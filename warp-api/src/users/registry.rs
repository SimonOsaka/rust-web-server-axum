use domain::RegistryUsers;
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;
use warp::{reject::custom, Filter, Rejection};

use crate::errors::RegistryError;
use crate::errors::ValidateError;
use crate::response::ErrorResponse;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct RegistryForm {
    #[validate(length(min = 2, max = 20, message = "username length(2-20)"))]
    username: String,
    #[validate(length(min = 8, max = 32, message = "password length(8-32)"))]
    password: String,
}

pub fn with_json_validate() -> impl Filter<Extract = (RegistryForm,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json().and_then(
        |json: RegistryForm| async move {
            match json.validate() {
                Ok(_) => Ok(json),
                Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
            }
        },
    ))
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
