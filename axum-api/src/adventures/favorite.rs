use axum::{extract::Extension, Json};
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    app_request::{AuthUser, ValidatedJson},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct FavoriteForm {
    #[validate(range(min = 1, message = "adventure_id not correct"))]
    adventure_id: i64,
}

#[derive(Serialize)]
pub struct FavoriteResponse {
    favorited: bool,
}

enum Action {
    Favorite,
    Unfavorite,
}

pub async fn favorite(
    ValidatedJson(form): ValidatedJson<FavoriteForm>,
    AuthUser(auth_user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<FavoriteResponse>, AppError> {
    process(form, AuthUser(auth_user), state, Action::Favorite).await
}

pub async fn unfavorite(
    ValidatedJson(form): ValidatedJson<FavoriteForm>,
    AuthUser(auth_user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<FavoriteResponse>, AppError> {
    process(form, AuthUser(auth_user), state, Action::Unfavorite).await
}

async fn process(
    form: FavoriteForm,
    AuthUser(auth_user): AuthUser,
    state: AppState,
    action: Action,
) -> Result<Json<FavoriteResponse>, AppError> {
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(auth_user.get_name()).await?;

    match action {
        Action::Favorite => {
            user.favorite(form.adventure_id, &state.favorites_manager)
                .await?;

            Ok(FavoriteResponse { favorited: true }.into())
        }
        Action::Unfavorite => {
            user.unfavorite(form.adventure_id, &state.favorites_manager)
                .await?;

            Ok(FavoriteResponse { favorited: false }.into())
        }
    }
}
