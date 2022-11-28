use crate::{app_response::AppError, response::AdventureResponse, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::AdventuresManager;
use macros::router;
use tracing::debug;
use vars::ID;

#[tracing::instrument(skip(state))]
#[router(path = "/api/adventures/:id")]
pub async fn get_adventure(
    Path(_id): Path<ID>,
    State(state): State<AppState>,
) -> Result<Json<AdventureResponse>, AppError> {
    debug!("_id: {:?}, state: {:?}", _id, state);

    let manager = &state.adventures_manager;
    let adventure = manager.get_adventure(_id).await?;
    let response = AdventureResponse::from(adventure);
    debug!("response: {:?}", &response);
    Ok(response.into())
}
