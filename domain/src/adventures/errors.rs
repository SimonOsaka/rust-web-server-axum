use types::ID;

use crate::DomainError;

#[derive(thiserror::Error, Debug)]
pub enum GetAdventureError {
    #[error("There is no adventure with id {adventure_id:?}.")]
    NotFound {
        adventure_id: ID,
        // source: DomainError,
    },
    #[error("Something went wrong.")]
    DomainError(#[from] DomainError),
}
