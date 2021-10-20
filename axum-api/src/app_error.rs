use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::{DomainError, GetAdventureError};
use serde_json::json;

use crate::app_response::{AppError, ErrorMessage};

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> AppError {
        AppError(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                })),
            )
                .into_response(),
        )
    }
}

impl From<GetAdventureError> for AppError {
    fn from(e: GetAdventureError) -> AppError {
        match &e {
            GetAdventureError::NotFound { .. } => AppError(
                (
                    StatusCode::NOT_FOUND,
                    Json(json!(ErrorMessage {
                        message: e.to_string(),
                        code: StatusCode::NOT_FOUND.as_u16(),
                    })),
                )
                    .into_response(),
            ),
            GetAdventureError::DomainError(_) => AppError(
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrorMessage {
                        message: e.to_string(),
                        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    })),
                )
                    .into_response(),
            ),
        }
    }
}
