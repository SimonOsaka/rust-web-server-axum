use domain::{manager::Manager, PlayListQuery};

use crate::{
    adventures::response::AdventuresResponse, response::ErrorResponse, routes::AuthUser, AppState,
};

pub async fn play_list_adventures(
    play_list: String,
    user: AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!(
        "user: {:?}, play_list: {:?}, state: {:?}",
        user, play_list, state
    );
    let manager = &state.manager;
    let query = PlayListQuery { play_list };
    let adventures = manager.find_adventures_by_play_list(query).await?;
    let response = AdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response)
}
