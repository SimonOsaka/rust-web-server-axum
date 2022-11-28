use axum::extract::State;
use axum::Json;
use domain::{RegistryUsers, UsersManager};
use macros::router;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app_error::RegistryError;
use crate::app_request::ValidatedJson;
use crate::app_response::AppError;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct RegistryForm {
    #[validate(length(min = 2, max = 20, code = "registry-valid-username"))]
    username: String,
    #[validate(length(min = 8, max = 32, code = "registry-valid-password"))]
    password: String,
}

#[derive(Serialize)]
pub struct RegistryResponse {
    username: String,
}

#[tracing::instrument(skip(state, registry_form))]
#[router(path = "/api/users/registry", method = "post")]
pub async fn registry(
    State(state): State<AppState>,
    ValidatedJson(registry_form): ValidatedJson<RegistryForm>,
) -> Result<Json<RegistryResponse>, AppError> {
    let manager = &state.users_manager;
    if manager
        .get_user_by_username(registry_form.username.clone())
        .await
        .is_ok()
    {
        return Err(AppError::from(RegistryError::UserExist));
    }

    let roles = vec!["user".to_string()];

    let user = RegistryUsers {
        username: registry_form.username,
        password: registry_form.password,
        roles,
    };

    let user = manager.add_user(user).await?;

    Ok(RegistryResponse {
        username: user.username,
    }
    .into())
}
