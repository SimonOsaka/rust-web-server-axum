use axum::{
    extract::{Extension, Path},
    Json,
};
use domain::{AdventuresManager, PlayListQuery};

use crate::{
    app_request::AuthUser, app_response::AppError, response::AdventuresResponse, AppState,
};

pub async fn play_list_adventures(
    Path(play_list): Path<String>,
    AuthUser(user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<AdventuresResponse>, AppError> {
    debug!(
        "user: {:?}, play_list: {:?}, state: {:?}",
        user, play_list, state
    );
    let manager = &state.adventures_manager;
    let query = PlayListQuery { play_list };
    let adventures = manager.find_adventures_by_play_list(query).await?;
    let response = AdventuresResponse::from(adventures);
    Ok(response.into())
}
