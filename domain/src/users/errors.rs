use thiserror::Error;
use util::i18n::{i18n, i18n_with_vars};

use crate::DomainError;

#[derive(Error, Debug)]
pub enum GetUserError {
    #[error("{}", i18n_with_vars("user-not-found", vec![username.to_string()]))]
    NotFound { username: String },
    #[error("{}", i18n_with_vars("user-password-not-correct", vec![username.to_string()]))]
    PasswordNotCorrect { username: String },
    #[error("{}", i18n("something-wrong"))]
    DomainError(#[from] DomainError),
}
