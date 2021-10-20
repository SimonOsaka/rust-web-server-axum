use std::convert::Infallible;

use domain::{manager::Manager, PlayListQuery};
use warp::Reply;

use crate::{adventures::response::AdventuresResponse, response::ErrorResponse, AppState};

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
    let query = PlayListQuery { play_list };
    match manager.find_adventures_by_play_list(query).await {
        Ok(adventures) => {
            let response = AdventuresResponse::from(adventures);
            debug!("response: {:?}", &response);
            Ok(warp::reply::json(&response).into_response())
        }
        Err(e) => Ok(ErrorResponse::from(e).into_response()),
    }
}
