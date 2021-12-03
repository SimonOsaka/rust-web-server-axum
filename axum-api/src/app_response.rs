use axum::{body::BoxBody, http::Response, response::IntoResponse};
use serde::Serialize;

pub struct AppError(pub Response<BoxBody>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response<BoxBody> {
        self.0
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
