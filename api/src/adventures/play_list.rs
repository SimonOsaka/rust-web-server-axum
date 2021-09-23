use std::convert::Infallible;

use domain::manager::Manager;

use crate::{adventures::response::AdventuresResponse, AppState};

pub async fn play_list_adventures(
    play_list: String,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "token: {:?}, play_list: {:?}, state: {:?}",
        token, play_list, state
    );
    let manager = &state.manager;
    let query = domain::PlayListQuery { play_list };
    let adventures = manager.find_adventures_by_play_list(query).await.unwrap();
    let response = AdventuresResponse::from(adventures);

    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
