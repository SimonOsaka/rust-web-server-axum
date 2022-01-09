use std::num::ParseIntError;

use auth::JWTError;
use domain::{
    CreateAdventureError, DeleteAdventureError, DomainError, FavoriteError, GetAdventureError,
    GetUserError,
};
use i18n::i18n;
use thiserror::Error;
use validator::ValidationErrors;
use warp::hyper::StatusCode;

use crate::response::{ErrorMessage, ErrorResponse};

fn forbidden(str: String) -> ErrorResponse {
    error_response(str, StatusCode::FORBIDDEN)
}

fn internal_server_error(str: String) -> ErrorResponse {
    error_response(str, StatusCode::INTERNAL_SERVER_ERROR)
}

fn not_found(str: String) -> ErrorResponse {
    error_response(str, StatusCode::NOT_FOUND)
}

fn unauthorized(str: String) -> ErrorResponse {
    error_response(str, StatusCode::UNAUTHORIZED)
}

fn bad_request(str: String) -> ErrorResponse {
    error_response(str, StatusCode::BAD_REQUEST)
}

fn error_response(message: String, code: StatusCode) -> ErrorResponse {
    ErrorResponse(
        ErrorMessage {
            message,
            code: code.as_u16(),
        },
        code,
    )
}

#[derive(Debug)]
pub enum ValidateError {
    InvalidParam(ValidationErrors),
    ParsePathIntParam(ParseIntError),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("{}", i18n("login-password-not-correct"))]
    WrongPassword,
    #[error("{}", i18n("login-user-not-exist"))]
    UserNotExist,
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("{}", i18n("registry-user-exist"))]
    UserExist,
}

#[derive(Error, Debug)]
pub enum ChangeUsernameError {
    #[error("{}", i18n("user-name-exist"))]
    UsernameExist,
}

impl From<DomainError> for ErrorResponse {
    fn from(e: DomainError) -> ErrorResponse {
        internal_server_error(e.to_string())
    }
}

impl From<GetAdventureError> for ErrorResponse {
    fn from(e: GetAdventureError) -> ErrorResponse {
        match &e {
            GetAdventureError::NotFound { .. } => not_found(e.to_string()),
            GetAdventureError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<JWTError> for ErrorResponse {
    fn from(e: JWTError) -> Self {
        match &e {
            JWTError::Invalid => unauthorized(e.to_string()),
            JWTError::Missing => unauthorized(e.to_string()),
        }
    }
}

impl From<ValidateError> for ErrorResponse {
    fn from(e: ValidateError) -> Self {
        match &e {
            ValidateError::InvalidParam(v) => bad_request(v.to_string().replace("\n", " , ")),
            ValidateError::ParsePathIntParam(v) => bad_request(v.to_string()),
        }
    }
}

impl From<GetUserError> for ErrorResponse {
    fn from(e: GetUserError) -> Self {
        match &e {
            GetUserError::NotFound { .. } => not_found(e.to_string()),
            GetUserError::PasswordNotCorrect { .. } => forbidden(e.to_string()),
            GetUserError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<LoginError> for ErrorResponse {
    fn from(e: LoginError) -> Self {
        match &e {
            LoginError::WrongPassword => forbidden(e.to_string()),
            LoginError::UserNotExist => unauthorized(e.to_string()),
        }
    }
}

impl From<RegistryError> for ErrorResponse {
    fn from(e: RegistryError) -> Self {
        match &e {
            RegistryError::UserExist => forbidden(e.to_string()),
        }
    }
}

impl From<ChangeUsernameError> for ErrorResponse {
    fn from(e: ChangeUsernameError) -> Self {
        match &e {
            ChangeUsernameError::UsernameExist => forbidden(e.to_string()),
        }
    }
}

impl From<CreateAdventureError> for ErrorResponse {
    fn from(e: CreateAdventureError) -> Self {
        match &e {
            CreateAdventureError::AdventureNotFound { .. } => not_found(e.to_string()),
            CreateAdventureError::Exist => forbidden(e.to_string()),
            CreateAdventureError::AddDocuments => internal_server_error(e.to_string()),
            CreateAdventureError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<FavoriteError> for ErrorResponse {
    fn from(e: FavoriteError) -> Self {
        match &e {
            FavoriteError::AlreadyExist { .. } => forbidden(e.to_string()),
            FavoriteError::AdventureNotFound { .. } => not_found(e.to_string()),
            FavoriteError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}

impl From<DeleteAdventureError> for ErrorResponse {
    fn from(e: DeleteAdventureError) -> Self {
        match &e {
            DeleteAdventureError::AdventureNotFound { .. } => not_found(e.to_string()),
            DeleteAdventureError::NotOwner => forbidden(e.to_string()),
            DeleteAdventureError::DelDocuments => internal_server_error(e.to_string()),
            DeleteAdventureError::DomainError(_) => internal_server_error(e.to_string()),
        }
    }
}
