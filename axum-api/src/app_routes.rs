use crate::{
    adventures::get::get_adventure, app_index::index, list::list_adventures,
    play_list::play_list_adventures, sync::sync_adventure, tabs::tabs_adventures,
    version::version_update_adventures,
};
use axum::{
    body::{Body, BoxBody},
    handler::get,
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
        .route("/api/adventures/:id", get(get_adventure))
        .route("/api/adventures", get(list_adventures))
        .route(
            "/api/adventures/playlist/:play_list",
            get(play_list_adventures),
        )
        .route("/api/adventures/update", get(version_update_adventures))
        .route("/api/adventures/tabs", get(tabs_adventures))
        .route("/api/sync/:id", get(sync_adventure))
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
