use domain::manager::Manager;
use types::ID;

use crate::{adventures::response::AdventureResponse, response::ErrorResponse, AppState};

pub async fn get_adventure(
    _id: ID,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);

    let manager = &state.manager;
    let adventure = manager.get_adventure(_id).await?;
    let response = AdventureResponse::from(adventure);
    debug!("response: {:?}", &response);
    Ok(response)
}
