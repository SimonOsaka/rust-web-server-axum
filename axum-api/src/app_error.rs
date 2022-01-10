use auth::JWTError;
use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{CreateAdventureError, DomainError, FavoriteError, GetAdventureError, GetUserError};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

use crate::app_response::{AppError, ErrorMessage};

#[derive(Debug)]
pub enum ValidateError {
    InvalidParam(ValidationErrors),
    AxumQueryRejection(QueryRejection),
    AxumJsonRejection(JsonRejection),
    AxumPathRejection(PathRejection),
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

fn forbidden(str: String) -> AppError {
    error_response(str, StatusCode::FORBIDDEN)
}

fn internal_server_error(str: String) -> AppError {
    error_response(str, StatusCode::INTERNAL_SERVER_ERROR)
}

fn not_found(str: String) -> AppError {
    error_response(str, StatusCode::NOT_FOUND)
}

fn unauthorized(str: String) -> AppError {
    error_response(str, StatusCode::UNAUTHORIZED)
}

fn bad_request(str: String) -> AppError {
    error_response(str, StatusCode::BAD_REQUEST)
}

fn error_response(message: String, code: StatusCode) -> AppError {
    AppError(
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

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> AppError {
        internal_server_error(e.to_string())
    }
}

impl From<GetAdventureError> for AppError {
    fn from(e: GetAdventureError) -> AppError {
        match &e {
            GetAdventureError::NotFound { .. } => not_found(e.to_string()),
            GetAdventureError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<JWTError> for AppError {
    fn from(e: JWTError) -> AppError {
        match &e {
            JWTError::Invalid => unauthorized(e.to_string()),
            JWTError::Missing => unauthorized(e.to_string()),
        }
    }
}

impl From<ValidateError> for AppError {
    fn from(e: ValidateError) -> Self {
        match &e {
            ValidateError::InvalidParam(v) => bad_request(v.to_string().replace('\n', " , ")),
            ValidateError::AxumQueryRejection(v) => bad_request(v.to_string()),
            ValidateError::AxumJsonRejection(v) => bad_request(v.to_string()),
            ValidateError::AxumPathRejection(v) => bad_request(v.to_string()),
        }
    }
}

impl From<GetUserError> for AppError {
    fn from(e: GetUserError) -> Self {
        match &e {
            GetUserError::NotFound { .. } => not_found(e.to_string()),
            GetUserError::PasswordNotCorrect { .. } => forbidden(e.to_string()),
            GetUserError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<LoginError> for AppError {
    fn from(e: LoginError) -> Self {
        match &e {
            LoginError::WrongPassword => forbidden(e.to_string()),
            LoginError::UserNotExist => unauthorized(e.to_string()),
        }
    }
}

impl From<RegistryError> for AppError {
    fn from(e: RegistryError) -> Self {
        match &e {
            RegistryError::UserExist => forbidden(e.to_string()),
        }
    }
}

impl From<ChangeUsernameError> for AppError {
    fn from(e: ChangeUsernameError) -> Self {
        match &e {
            ChangeUsernameError::UsernameExist => forbidden(e.to_string()),
        }
    }
}

impl From<CreateAdventureError> for AppError {
    fn from(e: CreateAdventureError) -> Self {
        match &e {
            CreateAdventureError::AdventureNotFound { .. } => not_found(e.to_string()),
            CreateAdventureError::Exist => forbidden(e.to_string()),
            CreateAdventureError::AddDocuments => internal_server_error(e.to_string()),
            CreateAdventureError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<FavoriteError> for AppError {
    fn from(e: FavoriteError) -> Self {
        match &e {
            FavoriteError::AlreadyExist { .. } => forbidden(e.to_string()),
            FavoriteError::DomainError(_) => internal_server_error(e.to_string()),
            FavoriteError::AdventureNotFound { .. } => not_found(e.to_string()),
        }
    }
}
