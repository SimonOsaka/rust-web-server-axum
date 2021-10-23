use domain::{manager::Manager, PlayListQuery};

use crate::{adventures::response::AdventuresResponse, response::ErrorResponse, AppState};

pub async fn play_list_adventures(
    play_list: String,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!(
        "token: {:?}, play_list: {:?}, state: {:?}",
        token, play_list, state
    );
    let manager = &state.manager;
    let query = PlayListQuery { play_list };
    let adventures = manager.find_adventures_by_play_list(query).await?;
    let response = AdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response)
}
