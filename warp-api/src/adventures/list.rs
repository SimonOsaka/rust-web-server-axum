use std::convert::Infallible;

use domain::manager::Manager;
use serde::Deserialize;
use warp::Reply;

use crate::{adventures::response::AdventuresResponse, response::ErrorResponse, AppState};

#[derive(Default, Deserialize, Debug, Clone)]
pub struct AdventuresQueryReq {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

impl From<AdventuresQueryReq> for domain::AdventuresQuery {
    fn from(ad: AdventuresQueryReq) -> Self {
        Self {
            item_id: ad.item_id,
            limit: ad.limit,
            offset: ad.offset,
            province_key: ad.province_key,
        }
    }
}

pub async fn list_adventures(
    token: Option<String>,
    query: AdventuresQueryReq,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, query: {:?}, state: {:?}", token, query, state);
    let manager = &state.manager;
    match manager.find_adventures(query.into()).await {
        Ok(adventures) => {
            let response = AdventuresResponse::from(adventures);
            debug!("response: {:?}", &response);
            Ok(warp::reply::json(&response).into_response())
        }
        Err(e) => Ok(ErrorResponse::from(e).into_response()),
    }
}
