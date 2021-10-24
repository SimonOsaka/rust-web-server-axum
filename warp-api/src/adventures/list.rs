use domain::manager::Manager;
use serde::Deserialize;

use crate::{
    adventures::response::AdventuresResponse, response::ErrorResponse, routes::AuthUser, AppState,
};

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
    AuthUser(user): AuthUser,
    query: AdventuresQueryReq,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("user: {:?}, query: {:?}, state: {:?}", user, query, state);
    let manager = &state.manager;
    let adventures = manager.find_adventures(query.into()).await?;
    let response = AdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response)
}
