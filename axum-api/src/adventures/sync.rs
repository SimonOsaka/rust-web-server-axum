use axum::{
    extract::{Extension, Path},
    Json,
};
use domain::AdventuresManager;
use tracing::debug;
use types::ID;

use crate::{app_request::AuthUser, app_response::AppError, AppState};

#[tracing::instrument(skip(user, state))]
pub async fn sync_adventure(
    Path(_id): Path<ID>,
    AuthUser(user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<bool>, AppError> {
    debug!("user: {:?}, _id: {:?}, state: {:?}", user, _id, state);
    let manager = &state.adventures_manager;
    let result = manager.sync_db_to_documents(_id).await?;
    Ok(result.into())
}
