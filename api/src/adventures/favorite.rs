use axum::{extract::Extension, Json};
use domain::UsersManager;
use macros::router;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    app_request::{JwtAuth, ValidatedPath},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct FavoriteForm {
    #[validate(range(min = 1, code = "adventure-favorite-valid-adventure_id"))]
    id: i64,
}

#[derive(Serialize)]
pub struct FavoriteResponse {
    favorited: bool,
}

enum Action {
    Favorite,
    Unfavorite,
}

#[tracing::instrument(skip(auth_user, state))]
#[router(path = "/api/adventures/:id/favorite", method = "post")]
pub async fn favorite(
    ValidatedPath(form): ValidatedPath<FavoriteForm>,
    JwtAuth(auth_user): JwtAuth,
    Extension(state): Extension<AppState>,
) -> Result<Json<FavoriteResponse>, AppError> {
    process(form, JwtAuth(auth_user), state, Action::Favorite).await
}

#[tracing::instrument(skip(auth_user, state))]
#[router(path = "/api/adventures/:id/unfavorite", method = "post")]
pub async fn unfavorite(
    ValidatedPath(form): ValidatedPath<FavoriteForm>,
    JwtAuth(auth_user): JwtAuth,
    Extension(state): Extension<AppState>,
) -> Result<Json<FavoriteResponse>, AppError> {
    process(form, JwtAuth(auth_user), state, Action::Unfavorite).await
}

async fn process(
    form: FavoriteForm,
    JwtAuth(auth_user): JwtAuth,
    state: AppState,
    action: Action,
) -> Result<Json<FavoriteResponse>, AppError> {
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(auth_user.get_name()).await?;

    match action {
        Action::Favorite => {
            user.favorite(form.id, &state.favorites_manager).await?;

            Ok(FavoriteResponse { favorited: true }.into())
        }
        Action::Unfavorite => {
            user.unfavorite(form.id, &state.favorites_manager).await?;

            Ok(FavoriteResponse { favorited: false }.into())
        }
    }
}
