use i18n::{i18n, i18n_with_vars};
use thiserror::Error;
use types::ID;

use crate::DomainError;

#[derive(Debug, Error)]
pub enum FavoriteError {
    #[error("{}", i18n_with_vars("favorite-already-exist",vec![user_id.to_string(), adventure_id.to_string()]))]
    AlreadyExist { user_id: ID, adventure_id: ID },

    #[error("{}", i18n_with_vars("favorite-adventure-not-found",vec![adventure_id.to_string()]))]
    AdventureNotFound { adventure_id: ID },

    #[error("{}", i18n("something-wrong"))]
    DomainError(#[from] DomainError),
}
