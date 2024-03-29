use axum::extract::State;
use domain::{adventures::DeleteAdventureError, UsersManager};
use hyper::StatusCode;
use macros::router;
use serde::Deserialize;
use tracing::debug;
use validator::Validate;

use crate::{
    app_request::{JwtAuth, ValidatedPath},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct DeleteAdventureReq {
    #[validate(range(min = 1, code = "adventure-delete-valid-adventure_id"))]
    pub id: i64,
}

#[tracing::instrument(skip(user, state))]
#[router(path = "/api/adventures/:id", method = "delete")]
pub async fn delete_adventure(
    JwtAuth(user): JwtAuth,
    State(state): State<AppState>,
    ValidatedPath(req): ValidatedPath<DeleteAdventureReq>,
) -> Result<StatusCode, AppError> {
    debug!("req {:?}", req);
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(user.get_name()).await?;

    let result = user
        .delete_adventure(req.id, &state.adventures_manager)
        .await;

    match result {
        Ok(_b) => Ok(StatusCode::OK),
        Err(_e) => match _e {
            _e @ DeleteAdventureError::AdventureNotFound { .. } => {
                Ok(StatusCode::OK)
            }
            _e @ DeleteAdventureError::DelDocuments => {
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
            _e @ DeleteAdventureError::NotOwner => Ok(StatusCode::FORBIDDEN),
            _e @ DeleteAdventureError::DomainError(_) => {
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}
