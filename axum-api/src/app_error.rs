use auth::JWTError;
use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{CreateAdventureError, DomainError, GetAdventureError, GetUserError};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

use crate::app_response::{AppError, ErrorMessage};

#[derive(Debug)]
pub enum ValidateError {
    InvalidParam(ValidationErrors),
    AxumQueryRejection(QueryRejection),
    AxumJsonRejection(JsonRejection),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Password is not correct")]
    WrongPassword,
    #[error("Login user doesn't exist")]
    UserNotExist,
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Registry user exist")]
    UserExist,
}

#[derive(Error, Debug)]
pub enum ChangeUsernameError {
    #[error("Username exist")]
    UsernameExist,
}

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

impl From<JWTError> for AppError {
    fn from(e: JWTError) -> AppError {
        match &e {
            JWTError::Invalid => AppError(
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!(ErrorMessage {
                        message: e.to_string(),
                        code: StatusCode::UNAUTHORIZED.as_u16(),
                    })),
                )
                    .into_response(),
            ),
            JWTError::Missing => AppError(
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!(ErrorMessage {
                        message: e.to_string(),
                        code: StatusCode::UNAUTHORIZED.as_u16(),
                    })),
                )
                    .into_response(),
            ),
        }
    }
}

impl From<ValidateError> for AppError {
    fn from(e: ValidateError) -> Self {
        match &e {
            ValidateError::InvalidParam(v) => AppError(
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!(ErrorMessage {
                        message: v.to_string().replace("\n", " , "),
                        code: StatusCode::BAD_REQUEST.as_u16(),
                    })),
                )
                    .into_response(),
            ),
            ValidateError::AxumQueryRejection(v) => AppError(
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!(ErrorMessage {
                        message: v.to_string(),
                        code: StatusCode::BAD_REQUEST.as_u16(),
                    })),
                )
                    .into_response(),
            ),
            ValidateError::AxumJsonRejection(v) => AppError(
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!(ErrorMessage {
                        message: v.to_string(),
                        code: StatusCode::BAD_REQUEST.as_u16(),
                    })),
                )
                    .into_response(),
            ),
        }
    }
}

impl From<GetUserError> for AppError {
    fn from(e: GetUserError) -> Self {
        match &e {
            GetUserError::NotFound { .. } => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::NOT_FOUND.as_u16(),
                }))
                .into_response(),
            ),
            GetUserError::PasswordNotCorrect { .. } => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                }))
                .into_response(),
            ),
            GetUserError::DomainError(_) => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                }))
                .into_response(),
            ),
        }
    }
}

impl From<LoginError> for AppError {
    fn from(e: LoginError) -> Self {
        match &e {
            LoginError::WrongPassword => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                }))
                .into_response(),
            ),
            LoginError::UserNotExist => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                }))
                .into_response(),
            ),
        }
    }
}

impl From<RegistryError> for AppError {
    fn from(e: RegistryError) -> Self {
        match &e {
            RegistryError::UserExist => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                }))
                .into_response(),
            ),
        }
    }
}

impl From<ChangeUsernameError> for AppError {
    fn from(e: ChangeUsernameError) -> Self {
        match &e {
            ChangeUsernameError::UsernameExist => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                }))
                .into_response(),
            ),
        }
    }
}

impl From<CreateAdventureError> for AppError {
    fn from(e: CreateAdventureError) -> Self {
        match &e {
            CreateAdventureError::AdventureNotFound { .. } => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::NOT_FOUND.as_u16(),
                }))
                .into_response(),
            ),
            CreateAdventureError::Exist => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                }))
                .into_response(),
            ),
            CreateAdventureError::AddDocuments => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                }))
                .into_response(),
            ),
            CreateAdventureError::DomainError(_) => AppError(
                Json(json!(ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                }))
                .into_response(),
            ),
        }
    }
}
