use domain::AdventuresManager;
use types::ID;

use crate::{response::ErrorResponse, routes::AuthUser, AppState};

pub async fn sync_adventure(
    _id: ID,
    user: AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("user: {:?}, _id: {:?}, state: {:?}", user, _id, state);
    let manager = &state.adventures_manager;
    let result = manager.sync_db_to_documents(_id).await?;
    Ok(warp::reply::json(&result))
}
