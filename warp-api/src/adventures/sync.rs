use domain::manager::Manager;
use types::ID;

use crate::{response::ErrorResponse, AppState};

pub async fn sync_adventure(
    _id: ID,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);
    let manager = &state.manager;
    let result = manager.sync_db_to_documents(_id).await?;
    Ok(warp::reply::json(&result))
}
