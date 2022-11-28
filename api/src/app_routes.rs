use crate::{
    app_response::{AppError, ErrorMessage},
    AppState,
};
use axum::{
    body::{Body, Bytes},
    error_handling::HandleErrorLayer,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Json, Router,
};
use hyper::{Request, StatusCode};
use serde_json::json;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

pub fn routes(state: AppState) -> Router {
    // don't change layer order, or errors happen...
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(30))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(print_request_response));

    Router::new()
        .merge(crate::app_index::get_index())
        // adventures
        .merge(crate::get::get_get_adventure())
        .merge(crate::delete::delete_delete_adventure())
        .merge(crate::list::get_list_adventures())
        .merge(crate::journey::post_journey())
        .merge(crate::play_list::get_play_list_adventures())
        .merge(crate::version::get_version_update_adventures())
        .merge(crate::tabs::get_tabs_adventures())
        .merge(crate::favorite::post_favorite())
        .merge(crate::favorite::post_unfavorite())
        .merge(crate::my_list::get_my_list_adventures())
        // sync
        .merge(crate::sync::get_sync_adventure())
        // users
        .merge(crate::registry::post_registry())
        .merge(crate::login::post_login())
        .merge(crate::me::get_me())
        .merge(crate::change_password::put_change_password())
        .merge(crate::change_username::put_change_username())
        .merge(crate::excel::get_download())
        .layer(middleware_stack.into_inner())
        .with_state(state)
}

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
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
