use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::response::ErrorResponse;
use crate::routes::AuthUser;
use crate::AppState;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct FavoriteForm {
    #[validate(range(min = 1, message = "adventure_id not correct"))]
    #[cfg(any(feature = "postgres"))]
    adventure_id: i64,
    #[cfg(any(feature = "mysql"))]
    adventure_id: u64,
}

#[derive(Serialize)]
struct FavoriteResponse {
    favorited: bool,
}

enum Action {
    Favorite,
    Unfavorite,
}

pub async fn favorite(
    form: FavoriteForm,
    AuthUser(auth_user): AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    process(form, AuthUser(auth_user), state, Action::Favorite).await
}

pub async fn unfavorite(
    form: FavoriteForm,
    AuthUser(auth_user): AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    process(form, AuthUser(auth_user), state, Action::Unfavorite).await
}

async fn process(
    form: FavoriteForm,
    AuthUser(auth_user): AuthUser,
    state: AppState,
    action: Action,
) -> Result<impl warp::Reply, ErrorResponse> {
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(auth_user.get_name()).await?;

    match action {
        Action::Favorite => {
            user.favorite(form.adventure_id, &state.favorites_manager)
                .await?;

            Ok(warp::reply::json(&FavoriteResponse { favorited: true }))
        }
        Action::Unfavorite => {
            user.unfavorite(form.adventure_id, &state.favorites_manager)
                .await?;

            Ok(warp::reply::json(&FavoriteResponse { favorited: false }))
        }
    }
}
