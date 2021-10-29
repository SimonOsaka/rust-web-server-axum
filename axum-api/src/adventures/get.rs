use crate::{app_request::AuthUser, app_response::AppError, response::AdventureResponse, AppState};
use axum::{
    extract::{Extension, Path},
    Json,
};
use domain::AdventuresManager;
use types::ID;

pub async fn get_adventure(
    Path(_id): Path<ID>,
    AuthUser(user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<AdventureResponse>, AppError> {
    debug!("_id: {:?}, user: {:?}, state: {:?}", _id, user, state);

    let manager = &state.manager;
    let adventure = manager.get_adventure(_id).await?;
    let response = AdventureResponse::from(adventure);
    debug!("response: {:?}", &response);
    Ok(response.into())
}
