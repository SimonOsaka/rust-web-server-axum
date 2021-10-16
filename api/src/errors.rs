use domain::{DomainError, GetAdventureError};
use warp::http::response::Response;
use warp::reply::json;
use warp::Reply;
use warp::{hyper::StatusCode, reply::with_status};

use crate::response::{ErrorMessage, ErrorResponse};

impl From<DomainError> for ErrorResponse {
    fn from(e: DomainError) -> ErrorResponse {
        ErrorResponse(
            with_status(
                json(&ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response(),
        )
    }
}

impl From<GetAdventureError> for ErrorResponse {
    fn from(e: GetAdventureError) -> ErrorResponse {
        match &e {
            GetAdventureError::NotFound { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::NOT_FOUND).into_response(),
            ),
            GetAdventureError::DomainError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}
