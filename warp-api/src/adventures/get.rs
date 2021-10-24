use domain::manager::Manager;
use types::ID;

use crate::{
    adventures::response::AdventureResponse, response::ErrorResponse, routes::AuthUser, AppState,
};

pub async fn get_adventure(
    _id: ID,
    user: AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("user: {:?}, _id: {:?}, state: {:?}", user, _id, state);

    let manager = &state.manager;
    let adventure = manager.get_adventure(_id).await?;
    let response = AdventureResponse::from(adventure);
    debug!("response: {:?}", &response);
    Ok(response)
}
