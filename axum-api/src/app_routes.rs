use crate::{
    adventures::get::get_adventure,
    app_index::index,
    app_response::{AppError, ErrorMessage},
    change_password::change_password,
    change_username::change_username,
    delete::delete_adventure,
    favorite::{favorite, unfavorite},
    journey::journey,
    list::list_adventures,
    login::login,
    me::me,
    play_list::play_list_adventures,
    registry::registry,
    sync::sync_adventure,
    tabs::tabs_adventures,
    version::version_update_adventures,
};
use axum::{
    body::{Body, BoxBody, Bytes},
    error_handling::HandleErrorLayer,
    response::IntoResponse,
    routing::{get, post, put},
    AddExtensionLayer, Json, Router,
};
use hyper::{Request, Response, StatusCode};
use serde_json::json;
use std::time::Duration;
use tower::{filter::AsyncFilterLayer, util::AndThenLayer, BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;

pub fn routes(state: AppState) -> Router {
    // don't change layer order, or errors happen...
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .layer(TraceLayer::new_for_http())
        .layer(AsyncFilterLayer::new(map_request))
        .layer(AndThenLayer::new(map_response))
        .timeout(Duration::from_secs(30))
        .layer(AddExtensionLayer::new(state));

    let r = Router::new()
        .route("/", get(index))
        // adventures
        .route(
            "/api/adventures/:id",
            get(get_adventure).delete(delete_adventure),
        )
        .route("/api/adventures", get(list_adventures).post(journey))
        .route(
            "/api/adventures/playlist/:play_list",
            get(play_list_adventures),
        )
        .route("/api/adventures/update", get(version_update_adventures))
        .route("/api/adventures/tabs", get(tabs_adventures))
        .route("/api/adventures/:id/favorite", post(favorite))
        .route("/api/adventures/:id/unfavorite", post(unfavorite))
        // sync
        .route("/api/sync/:id", get(sync_adventure))
        // users
        .route("/api/users/registry", post(registry))
        .route("/api/users/login", post(login))
        .route("/api/users/me", get(me))
        .route("/api/users/password", put(change_password))
        .route("/api/users/username", put(change_username))
        .layer(middleware_stack);

    r
}

async fn map_request(req: Request<Body>) -> Result<Request<Body>, BoxError> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    Ok(req)
}

async fn map_response(res: Response<BoxBody>) -> Result<Response<Body>, BoxError> {
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, BoxError>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: Into<BoxError>,
{
    let bytes = hyper::body::to_bytes(body).await.map_err(Into::into)?;
    if let Ok(body) = std::str::from_utf8(&bytes) {
        debug!("{} body = {:?}", direction, body);
    }
    Ok(bytes)
}

fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        Ok(StatusCode::REQUEST_TIMEOUT)
    } else {
        Err(AppError(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(ErrorMessage {
                    message: format!("Unhandled internal error: {}", error),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                })),
            )
                .into_response(),
        ))
    }
}
