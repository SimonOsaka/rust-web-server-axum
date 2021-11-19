use domain::{AdventuresManager, PlayListQuery};
use tracing::debug;

use crate::{adventures::response::AdventuresResponse, response::ErrorResponse, AppState};

#[tracing::instrument(skip(state))]
pub async fn play_list_adventures(
    play_list: String,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("play_list: {:?}, state: {:?}", play_list, state);
    let manager = &state.adventures_manager;
    let query = PlayListQuery { play_list };
    let adventures = manager.find_adventures_by_play_list(query).await?;
    let response = AdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response)
}
