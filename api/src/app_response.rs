use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;

pub struct AppError(pub Response);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.0
    }
}

impl AppError {
    pub(crate) fn forbidden(str: String) -> Self {
        Self::error_response(str, StatusCode::FORBIDDEN)
    }

    pub(crate) fn internal_server_error(str: String) -> Self {
        Self::error_response(str, StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub(crate) fn not_found(str: String) -> Self {
        Self::error_response(str, StatusCode::NOT_FOUND)
    }

    pub(crate) fn unauthorized(str: String) -> Self {
        Self::error_response(str, StatusCode::UNAUTHORIZED)
    }

    pub(crate) fn bad_request(str: String) -> Self {
        Self::error_response(str, StatusCode::BAD_REQUEST)
    }

    pub(crate) fn error_response(message: String, code: StatusCode) -> Self {
        Self(
            (
                code,
                Json(json!(ErrorMessage {
                    message,
                    code: code.as_u16(),
                })),
            )
                .into_response(),
        )
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
