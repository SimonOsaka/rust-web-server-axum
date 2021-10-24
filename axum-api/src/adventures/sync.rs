use axum::{
    extract::{Extension, Path},
    Json,
};
use domain::manager::Manager;
use types::ID;

use crate::{app_request::AuthUser, app_response::AppError, AppState};

pub async fn sync_adventure(
    Path(_id): Path<ID>,
    AuthUser(user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<bool>, AppError> {
    debug!("user: {:?}, _id: {:?}, state: {:?}", user, _id, state);
    let manager = &state.manager;
    let result = manager.sync_db_to_documents(_id).await?;
    Ok(result.into())
}
