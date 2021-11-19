use crate::request::PathValidate;
use domain::{DeleteAdventureError, UsersManager};
use serde::Deserialize;
use tracing::debug;
use validator::Validate;
use warp::hyper::StatusCode;

use crate::{response::ErrorResponse, routes::AuthUser, AppState};
// https://github.com/gnosis/gp-v2-services/blob/main/crates/orderbook/src/api/get_markets.rs
#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct DeleteAdventureReq {
    #[validate(range(min = 1, message = "adventure_id not correct"))]
    pub adventure_id: i64,
}

impl PathValidate for DeleteAdventureReq {}

#[tracing::instrument(skip(user, state))]
pub async fn delete_adventure(
    req: DeleteAdventureReq,
    AuthUser(user): AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("req {:?}", req);
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(user.get_name()).await?;

    let result = user
        .delete_adventure(req.adventure_id, &state.adventures_manager)
        .await;

    match result {
        Ok(_b) => Ok(StatusCode::OK),
        Err(_e) => match _e {
            _e @ DeleteAdventureError::AdventureNotFound { .. } => Ok(StatusCode::OK),
            _e @ DeleteAdventureError::DelDocuments => Ok(StatusCode::INTERNAL_SERVER_ERROR),
            _e @ DeleteAdventureError::NotOwner => Ok(StatusCode::FORBIDDEN),
            _e @ DeleteAdventureError::DomainError(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
