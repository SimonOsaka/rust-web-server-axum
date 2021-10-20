use std::convert::Infallible;

use domain::manager::Manager;
use types::ID;

use crate::AppState;

pub async fn sync_adventure(
    _id: ID,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);
    let manager = &state.manager;
    let result = manager.sync_db_to_documents(_id).await;
    let r = match result {
        Ok(b) => b,
        Err(_) => false,
    };
    Ok(warp::reply::json(&r))
}
