use axum::{
    body::{Bytes, Full},
    http::Response,
    response::IntoResponse,
};
use serde::Serialize;
use std::convert::Infallible;

pub struct AppError(pub Response<Full<Bytes>>);

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        self.0
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
