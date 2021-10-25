use auth::{decode_token, role_view, Claims, JWTError};
use types::ID;
use warp::header::headers_cloned;
use warp::hyper::header::AUTHORIZATION;
use warp::hyper::HeaderMap;
use warp::{self, get, Reply};
use warp::{Filter, Rejection};

use crate::get::get_adventure;
use crate::index::index;
use crate::list::{list_adventures, with_query_validate, AdventuresQueryReq};
use crate::login;
use crate::play_list::play_list_adventures;
use crate::response::ErrorResponse;
use crate::sync::sync_adventure;
use crate::tabs::tabs_adventures;
use crate::version::version_update_adventures;
use crate::AppState;

pub fn routes(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .map(index)
        .or(warp::path!("api" / "adventures")
            .and(get())
            .and(with_auth())
            .and(with_query_validate())
            .and(with_state(state.clone()))
            .and_then(
                |user: AuthUser, query: AdventuresQueryReq, state: AppState| async move {
                    list_adventures(user, query, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "adventures" / ID)
            .and(warp::get())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|_id: ID, user: AuthUser, state: AppState| async move {
                get_adventure(_id, user, state)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        .or(warp::path!("api" / "adventures" / "tabs")
            .and(warp::get())
            .and(with_auth())
            .and_then(tabs_adventures))
        .or(warp::path!("api" / "adventures" / "update")
            .and(warp::get())
            .and(with_auth())
            .and(warp::query())
            .and_then(version_update_adventures))
        .or(warp::path!("api" / "adventures" / "playlist" / String)
            .and(warp::get())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(
                |play_list: String, user: AuthUser, state: AppState| async move {
                    play_list_adventures(play_list, user, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        //sync
        .or(warp::path!("api" / "sync" / ID)
            .and(warp::get())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|_id: ID, user: AuthUser, state: AppState| async move {
                sync_adventure(_id, user, state)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        //users
        .or(warp::path!("api" / "users" / "login")
            .and(warp::post())
            .and_then(login))
        .recover(handle_rejection)
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

#[derive(Debug)]
pub struct AuthUser(pub Claims);

fn with_auth() -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    headers_cloned().and_then(|x: HeaderMap| async move {
        match x.get(AUTHORIZATION) {
            Some(k) => match k.to_str().ok().and_then(|x| decode_token(x).ok()) {
                Some(k) => Ok(AuthUser(k)),
                None => Err(warp::reject::custom(ErrorResponse::from(JWTError::Invalid))),
            },
            // for no login user
            None => Ok(AuthUser(role_view())), //Err(warp::reject::custom(ErrorResponse::from(JWTError::Missing))),
        }
    })
}

async fn handle_rejection(
    err: warp::reject::Rejection,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    if let Some(e) = err.find::<ErrorResponse>() {
        return Ok(e.clone().into_response());
    }

    Err(err)
}
