use types::ID;
use warp::{self, Reply, get, Filter, Rejection};
use crate::my_list::my_list_adventures;
use crate::request::{PathValidate, with_query_validate, with_auth, AuthUser, with_json_validate, handle_rejection, with_state};
use crate::delete::{DeleteAdventureReq, delete_adventure};
use crate::favorite::{FavoriteForm, favorite, unfavorite};
use crate::get::get_adventure;
use crate::index::index;
use crate::journey::{journey, JourneyForm};
use crate::list::{list_adventures, AdventuresQueryReq};
use crate::play_list::play_list_adventures;

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
        .or(warp::path!("api" / "adventures" / "my")
            .and(get())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(
                |user: AuthUser, state: AppState| async move {
                    my_list_adventures(user, state)
                        .await
                        .map_err(|e| warp::reject::custom(e))
                },
            ))
        .or(warp::path!("api" / "adventures" / ID)
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(|_id: ID, state: AppState| async move {
                get_adventure(_id, state)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        .or(warp::path!("api" / "adventures" / ID)
            .and(warp::delete())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|_id: ID, user: AuthUser,state: AppState| async move {
                let v = DeleteAdventureReq {
                    adventure_id: _id
                };
                let req = v.valid().await?;
                delete_adventure(req, user, state)
                        .await
                        .map_err(|e| warp::reject::custom(e)) 
            }))
        .or(warp::path!("api" / "adventures" / "tabs")
            .and(warp::get())
            .and_then(tabs_adventures))
        .or(warp::path!("api" / "adventures" / "update")
            .and(warp::get())
            .and(warp::query())
            .and_then(version_update_adventures))
        .or(warp::path!("api" / "adventures" / "playlist" / String)
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(
                |play_list: String, state: AppState| async move {
                    play_list_adventures(play_list, state)
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
        .or(warp::path!("api" / "adventures" / ID / "favorite")
            .and(warp::post())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|_id: ID, user: AuthUser, state: AppState| async move {
                let v = FavoriteForm {
                    adventure_id: _id,
                };
                let form = v.valid().await?;
                favorite(form, user, state) 
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }))
        .or(warp::path!("api" / "adventures" / ID / "unfavorite")
            .and(warp::post())
            .and(with_auth())
            .and(with_state(state.clone()))
            .and_then(|_id: ID, user: AuthUser, state: AppState| async move {
                let v = FavoriteForm {
                    adventure_id: _id,
                };
                let form = v.valid().await?;
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
