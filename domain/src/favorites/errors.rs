use thiserror::Error;
use types::ID;

use crate::DomainError;

#[derive(Debug, Error)]
pub enum FavoriteError {
    #[error("favorite already exist, user id = {user_id:?} adventure id = {adventure_id:?}")]
    AlreadyExist { user_id: ID, adventure_id: ID },

    #[error("There is no adventure id = {adventure_id:?} exist")]
    AdventureNotFound { adventure_id: ID },

    #[error("Something went wrong.")]
    DomainError(#[from] DomainError),
}
