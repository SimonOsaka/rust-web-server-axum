use crate::{
    adventures::get::get_adventure, app_index::index, change_password::change_password,
    change_username::change_username, list::list_adventures, login::login, me::me,
    play_list::play_list_adventures, registry::registry, sync::sync_adventure,
    tabs::tabs_adventures, version::version_update_adventures,
};
use axum::{
    body::{Body, BoxBody},
    handler::{get, post, put},
    http::{Request, Response},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use serde::ser::StdError;
use std::convert::Infallible;
use tower::{filter::AsyncFilterLayer, util::AndThenLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;

pub fn routes(
    state: AppState,
) -> Router<BoxRoute<Body, Box<(dyn StdError + Send + Sync + 'static)>>> {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(AsyncFilterLayer::new(map_request))
        .layer(AndThenLayer::new(map_response))
        .layer(AddExtensionLayer::new(state))
        .into_inner();

    let r = Router::new()
        .route("/", get(index))
        // adventures
        .route("/api/adventures/:id", get(get_adventure))
        .route("/api/adventures", get(list_adventures))
        .route(
            "/api/adventures/playlist/:play_list",
            get(play_list_adventures),
        )
        .route("/api/adventures/update", get(version_update_adventures))
        .route("/api/adventures/tabs", get(tabs_adventures))
        .route("/api/sync/:id", get(sync_adventure))
        // users
        .route("/api/users/registry", post(registry))
        .route("/api/users/login", post(login))
        .route("/api/users/me", get(me))
        .route("/api/users/password", put(change_password))
        .route("/api/users/username", put(change_username))
        .layer(middleware_stack)
        .boxed();

    r
}

async fn map_request(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    Ok(req)
}

async fn map_response(res: Response<BoxBody>) -> Result<Response<BoxBody>, Infallible> {
    Ok(res)
}
