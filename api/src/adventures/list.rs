use axum::{extract::State, Json};
use domain::{adventures::AdventuresManager, adventures::AdventuresQuery};
use macros::router;
use serde::Deserialize;
use tracing::debug;
use validator::Validate;

use crate::{
    app_request::ValidatedQuery, app_response::AppError,
    response::AdventuresResponse, AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct AdventuresQueryReq {
    #[validate(custom(function = "crate::app_request::validate_item_id"))]
    pub item_id: u8,
    #[validate(range(min = 1, max = 20, code = "adventure-list-valid-limit"))]
    pub limit: Option<u32>,
    #[validate(range(min = 0, code = "adventure-list-valid-offset"))]
    pub offset: Option<u32>,
    #[validate(length(min = 2, code = "adventure-list-valid-province_key"))]
    pub province_key: Option<String>,
}

impl From<AdventuresQueryReq> for AdventuresQuery {
    fn from(ad: AdventuresQueryReq) -> Self {
        Self {
            item_id: ad.item_id,
            limit: ad.limit,
            offset: ad.offset,
            province_key: ad.province_key,
        }
    }
}

#[tracing::instrument(skip(state))]
#[router(path = "/api/adventures")]
pub async fn list_adventures(
    State(state): State<AppState>,
    ValidatedQuery(query): ValidatedQuery<AdventuresQueryReq>,
) -> Result<Json<AdventuresResponse>, AppError> {
    debug!("query: {:?}, state: {:?}", query, state);
    let manager = &state.adventures_manager;
    let adventures = manager.find_adventures(query.into()).await?;
    let response = AdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response.into())
}
