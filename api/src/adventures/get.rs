use std::convert::Infallible;

use domain::manager::Manager;
use types::ID;
use warp::{hyper::StatusCode, Reply};

use crate::{
    adventures::response::{AdventureResponse, Response404},
    AppState,
};

pub async fn get_adventure(
    _id: ID,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);
    let manager = &state.manager;
    if let Some(adventure) = manager.get_adventure_by_id(_id).await.unwrap() {
        let response = AdventureResponse::from(adventure);
        debug!("response: {:?}", &response);
        Ok(warp::reply::json(&response).into_response())
    } else {
        let json = warp::reply::json(&Response404 {
            message: "404".to_owned(),
        });
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND).into_response())
    }
}
