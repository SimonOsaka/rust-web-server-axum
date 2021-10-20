use std::convert::Infallible;

use domain::manager::Manager;
use types::ID;
use warp::Reply;

use crate::{
    adventures::response::AdventureResponse,
    // jwt_token::decode_token,
    response::ErrorResponse,
    AppState,
};

pub async fn get_adventure(
    _id: ID,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);
    // let t = token
    //     .map(|token| -> jsonwebtoken::errors::Result<ID> { Ok(decode_token(&token)?.id()) })
    //     .transpose()
    //     .unwrap();
    // debug!("decode token => {:?}", t);
    // t present any id
    // match t {
    // Some(x)=> .....,
    // Node=>...
    // }

    let manager = &state.manager;
    match manager.get_adventure_by_id(_id).await {
        Ok(x) => match x {
            // exist
            Some(adventure) => {
                let response = AdventureResponse::from(adventure);
                debug!("response: {:?}", &response);
                Ok(warp::reply::json(&response).into_response())
            }
            // not exist
            None => Ok(warp::http::status::StatusCode::NOT_FOUND.into_response()),
        },
        // something happens
        Err(e) => Ok(ErrorResponse::from(e).into_response()),
    }
}
