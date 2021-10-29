use thiserror::Error;

use crate::DomainError;

#[derive(Error, Debug)]
pub enum GetUserError {
    #[error("There is no user with username {username:?}.")]
    NotFound { username: String },
    #[error("Something went wrong.")]
    DomainError(#[from] DomainError),
}
