use domain::manager::Manager;
use serde::Deserialize;
use types::my_item_type_format::to_item_type_name;
use validator::{Validate, ValidationError};
use warp::{query, reject::custom, Filter, Rejection};

use crate::{
    adventures::response::AdventuresResponse, errors::ValidateError, response::ErrorResponse,
    routes::AuthUser, AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct AdventuresQueryReq {
    #[validate(custom(function = "validate_item_id", message = "item_id is not correct"))]
    pub item_id: u8,
    #[validate(range(min = 1, max = 20, message = "limit 1 - 20"))]
    pub limit: Option<u32>,
    #[validate(range(min = 0, message = "offset start at 0"))]
    pub offset: Option<u32>,
    #[validate(length(min = 2, message = "province_key at lease 2 chars"))]
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

pub fn with_query_validate(
) -> impl Filter<Extract = (AdventuresQueryReq,), Error = Rejection> + Clone {
    query::<AdventuresQueryReq>().and_then(|req: AdventuresQueryReq| async move {
        match req.validate() {
            Ok(_) => Ok(req),
            Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
        }
    })
}

fn validate_item_id(item_id: u8) -> Result<(), ValidationError> {
    if to_item_type_name(item_id.into()) == "" {
        return Err(ValidationError::new("item_id"));
    }

    Ok(())
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
