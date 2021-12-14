use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct AppError(pub Response);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.0
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
