use types::ID;
use warp::header::optional;
use warp::{self, get, query, Reply};
use warp::{Filter, Rejection};

use crate::get::get_adventure;
use crate::index::index;
use crate::list::AdventuresQueryReq;
use crate::play_list::play_list_adventures;
use crate::response::ErrorResponse;
use crate::sync::sync_adventure;
use crate::tabs::tabs_adventures;
use crate::version::version_update_adventures;
use crate::{list::list_adventures, AppState};

pub fn routes(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .map(index)
        .or(warp::path!("api" / "adventures")
            .and(get())
            .and(optional("Authorization"))
            .and(query())
            .and(with_state(state.clone()))
            .and_then(
                |token: Option<String>, query: AdventuresQueryReq, state: AppState| async move {
                    list_adventures(token, query, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "adventures" / ID)
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(with_state(state.clone()))
            .and_then(
                |_id: ID, token: Option<String>, state: AppState| async move {
                    get_adventure(_id, token, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "adventures" / "tabs")
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and_then(tabs_adventures))
        .or(warp::path!("api" / "adventures" / "update")
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(warp::query())
            .and_then(version_update_adventures))
        .or(warp::path!("api" / "adventures" / "playlist" / String)
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(with_state(state.clone()))
            .and_then(
                |play_list: String, token: Option<String>, state: AppState| async move {
                    play_list_adventures(play_list, token, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        //
        .or(warp::path!("api" / "sync" / ID)
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(with_state(state.clone()))
            .and_then(
                |_id: ID, token: Option<String>, state: AppState| async move {
                    sync_adventure(_id, token, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .recover(handle_rejection)
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_rejection(
    err: warp::reject::Rejection,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    if let Some(e) = err.find::<ErrorResponse>() {
        return Ok(e.clone().into_response());
    }

    Err(err)
}
