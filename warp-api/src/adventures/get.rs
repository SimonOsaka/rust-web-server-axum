use domain::AdventuresManager;
use types::ID;

use crate::{adventures::response::AdventureResponse, response::ErrorResponse, AppState};

pub async fn get_adventure(_id: ID, state: AppState) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("_id: {:?}, state: {:?}", _id, state);

    let manager = &state.adventures_manager;
    let adventure = manager.get_adventure(_id).await?;
    let response = AdventureResponse::from(adventure);
    debug!("response: {:?}", &response);
    Ok(response)
}
