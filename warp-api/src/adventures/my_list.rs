use domain::UsersManager;
use tracing::debug;

use crate::{
    adventures::response::MyAdventuresResponse, request::AuthUser, response::ErrorResponse,
    AppState,
};

#[tracing::instrument(skip(user, state))]
pub async fn my_list_adventures(
    AuthUser(user): AuthUser,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("user: {:?}, state: {:?}", user, state);
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(user.get_name()).await?;
    let adventures = user.find_adventures(&state.adventures_manager).await?;
    let response = MyAdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response)
}
