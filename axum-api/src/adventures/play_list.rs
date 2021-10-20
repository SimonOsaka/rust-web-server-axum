use axum::{
    extract::{Extension, Path},
    Json,
};
use domain::{manager::Manager, PlayListQuery};

use crate::{
    app_request::JwtToken, app_response::AppError, response::AdventuresResponse, AppState,
};

pub async fn play_list_adventures(
    Path(play_list): Path<String>,
    JwtToken(token): JwtToken,
    Extension(state): Extension<AppState>,
) -> Result<Json<AdventuresResponse>, AppError> {
    debug!(
        "token: {:?}, play_list: {:?}, state: {:?}",
        token, play_list, state
    );
    let manager = &state.manager;
    let query = PlayListQuery { play_list };
    let adventures = manager.find_adventures_by_play_list(query).await?;
    let response = AdventuresResponse::from(adventures);
    Ok(response.into())
}
