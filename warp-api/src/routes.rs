use auth::{decode_token, Claims, JWTError};
use serde::de::DeserializeOwned;
use types::ID;
use validator::Validate;
use warp::header::headers_cloned;
use warp::hyper::header::AUTHORIZATION;
use warp::hyper::HeaderMap;
use warp::reject::custom;
use warp::{self, get, query, Reply};
use warp::{Filter, Rejection};

use crate::errors::ValidateError;
use crate::favorite::{FavoriteForm, favorite, unfavorite};
use crate::get::get_adventure;
use crate::index::index;
use crate::journey::{journey, JourneyForm};
use crate::list::{list_adventures, AdventuresQueryReq};
use crate::play_list::play_list_adventures;
use crate::response::ErrorResponse;
use crate::sync::sync_adventure;
use crate::tabs::tabs_adventures;
use crate::version::version_update_adventures;
use crate::{
    change_password, change_username, login, me, registry, AppState, ChangePasswordForm,
    ChangeUsernameForm, LoginForm, RegistryForm,
};

pub fn routes(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .map(index)
        .or(warp::path!("api" / "adventures")
            .and(get())
            .and(with_query_validate())
            .and(with_state(state.clone()))
            .and_then(
                |query: AdventuresQueryReq, state: AppState| async move {
                    list_adventures(query, state)
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
            .or(warp::path!("api" / "adventures")
            .and(warp::post())
            .and(with_json_validate())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(
                |form: JourneyForm, user: AuthUser, state: AppState| async move {
                    journey(form, user, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "adventures" / "favorite")
            .and(warp::post())
            .and(with_json_validate())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|form: FavoriteForm, user: AuthUser, state: AppState| async move {
                favorite(form, user, state) 
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        .or(warp::path!("api" / "adventures" / "unfavorite")
            .and(warp::post())
            .and(with_json_validate())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|form: FavoriteForm, user: AuthUser, state: AppState| async move {
                unfavorite(form, user, state) 
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
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
            .and(with_json_validate())
            .and(with_state(state.clone()))
            .and_then(|login_form: LoginForm, state: AppState| async move {
                login(login_form, state)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        .or(warp::path!("api" / "users" / "registry")
            .and(warp::post())
            .and(with_json_validate())
            .and(with_state(state.clone()))
            .and_then(|registry_form: RegistryForm, state: AppState| async move {
                registry(registry_form, state)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        .or(warp::path!("api" / "users" / "password")
            .and(warp::put())
            .and(with_auth())
            .and(with_json_validate())
            .and(with_state(state.clone()))
            .and_then(
                |user: AuthUser,change_password_form: ChangePasswordForm, state: AppState| async move {
                    change_password(user,change_password_form, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "users" / "username")
            .and(warp::put())
            .and(with_auth())
            .and(with_json_validate())
            .and(with_state(state.clone()))
            .and_then(
                |user: AuthUser,change_username_form: ChangeUsernameForm, state: AppState| async move {
                    change_username(user,change_username_form, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "users" / "me")
            .and(warp::get())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(
                |user: AuthUser,state: AppState| async move {
                    me(user, state)
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
            None =>
            //Ok(AuthUser(role_view())),
            {
                Err(warp::reject::custom(ErrorResponse::from(JWTError::Missing)))
            }
        }
    })
}

fn with_json_validate<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: Validate + std::marker::Send + DeserializeOwned,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json().and_then(
        |val: T| async move {
            match val.validate() {
                Ok(_) => Ok(val),
                Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
            }
        },
    ))
}

fn with_query_validate<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: 'static + Validate + std::marker::Send + DeserializeOwned,
{
    query::<T>().and_then(|req: T| async move {
        match req.validate() {
            Ok(_) => Ok(req),
            Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
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
