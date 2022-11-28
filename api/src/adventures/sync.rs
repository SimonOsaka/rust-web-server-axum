use axum::{
    extract::{Path, State},
    Json,
};
use domain::AdventuresManager;
use macros::router;
use tracing::debug;
use vars::ID;

use crate::{app_request::JwtAuth, app_response::AppError, AppState};

#[tracing::instrument(skip(user, state))]
#[router(path = "/api/sync/:id")]
pub async fn sync_adventure(
    Path(_id): Path<ID>,
    JwtAuth(user): JwtAuth,
    State(state): State<AppState>,
) -> Result<Json<bool>, AppError> {
    debug!("user: {:?}, _id: {:?}, state: {:?}", user, _id, state);
    let manager = &state.adventures_manager;
    let result = manager.sync_db_to_documents(_id).await?;
    Ok(result.into())
}
