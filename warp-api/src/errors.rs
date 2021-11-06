use auth::JWTError;
use domain::{CreateAdventureError, DomainError, FavoriteError, GetAdventureError, GetUserError};
use thiserror::Error;
use validator::ValidationErrors;
use warp::hyper::StatusCode;

use crate::response::{ErrorMessage, ErrorResponse};

#[derive(Debug)]
pub enum ValidateError {
    InvalidParam(ValidationErrors),
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

impl From<DomainError> for ErrorResponse {
    fn from(e: DomainError) -> ErrorResponse {
        ErrorResponse(
            ErrorMessage {
                message: e.to_string(),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            },
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<GetAdventureError> for ErrorResponse {
    fn from(e: GetAdventureError) -> ErrorResponse {
        match &e {
            GetAdventureError::NotFound { .. } => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::NOT_FOUND.as_u16(),
                },
                StatusCode::NOT_FOUND,
            ),
            GetAdventureError::DomainError(_) => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<JWTError> for ErrorResponse {
    fn from(e: JWTError) -> Self {
        match &e {
            JWTError::Invalid => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                },
                StatusCode::UNAUTHORIZED,
            ),
            JWTError::Missing => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                },
                StatusCode::UNAUTHORIZED,
            ),
        }
    }
}

impl From<ValidateError> for ErrorResponse {
    fn from(e: ValidateError) -> Self {
        match &e {
            ValidateError::InvalidParam(v) => ErrorResponse(
                ErrorMessage {
                    message: v.to_string().replace("\n", " , "),
                    code: StatusCode::BAD_REQUEST.as_u16(),
                },
                StatusCode::BAD_REQUEST,
            ),
        }
    }
}

impl From<GetUserError> for ErrorResponse {
    fn from(e: GetUserError) -> Self {
        match &e {
            GetUserError::NotFound { .. } => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::NOT_FOUND.as_u16(),
                },
                StatusCode::NOT_FOUND,
            ),
            GetUserError::PasswordNotCorrect { .. } => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                },
                StatusCode::FORBIDDEN,
            ),
            GetUserError::DomainError(_) => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<LoginError> for ErrorResponse {
    fn from(e: LoginError) -> Self {
        match &e {
            LoginError::WrongPassword => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                },
                StatusCode::FORBIDDEN,
            ),
            LoginError::UserNotExist => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                },
                StatusCode::UNAUTHORIZED,
            ),
        }
    }
}

impl From<RegistryError> for ErrorResponse {
    fn from(e: RegistryError) -> Self {
        match &e {
            RegistryError::UserExist => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                },
                StatusCode::FORBIDDEN,
            ),
        }
    }
}

impl From<ChangeUsernameError> for ErrorResponse {
    fn from(e: ChangeUsernameError) -> Self {
        match &e {
            ChangeUsernameError::UsernameExist => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                },
                StatusCode::FORBIDDEN,
            ),
        }
    }
}

impl From<CreateAdventureError> for ErrorResponse {
    fn from(e: CreateAdventureError) -> Self {
        match &e {
            CreateAdventureError::AdventureNotFound { .. } => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::NOT_FOUND.as_u16(),
                },
                StatusCode::NOT_FOUND,
            ),
            CreateAdventureError::Exist => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                },
                StatusCode::FORBIDDEN,
            ),
            CreateAdventureError::AddDocuments => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            CreateAdventureError::DomainError(_) => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<FavoriteError> for ErrorResponse {
    fn from(e: FavoriteError) -> Self {
        match &e {
            FavoriteError::AlreadyExist { .. } => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::FORBIDDEN.as_u16(),
                },
                StatusCode::FORBIDDEN,
            ),
            FavoriteError::DomainError(_) => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
