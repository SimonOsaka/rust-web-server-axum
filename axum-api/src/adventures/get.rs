use crate::{app_request::JwtToken, app_response::AppError, response::AdventureResponse, AppState};
use axum::{
    extract::{Extension, Path},
    Json,
};
use domain::manager::Manager;
use types::ID;

pub async fn get_adventure(
    Path(_id): Path<ID>,
    JwtToken(token): JwtToken,
    Extension(state): Extension<AppState>,
) -> Result<Json<AdventureResponse>, AppError> {
    debug!("_id: {:?}, token: {:?}, state: {:?}", _id, token, state);

    let manager = &state.manager;
    let adventure = manager.get_adventure(_id).await?;
    let response = AdventureResponse::from(adventure);
    debug!("response: {:?}", &response);
    Ok(response.into())
}
