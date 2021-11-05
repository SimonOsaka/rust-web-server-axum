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

#[derive(thiserror::Error, Debug)]
pub enum CreateAdventureError {
    #[error("There is no adventure with id {adventure_id:?}.")]
    AdventureNotFound {
        adventure_id: ID,
        // source: DomainError,
    },
    #[error("Adventure exist")]
    Exist,
    #[error("Add document error")]
    AddDocuments,
    #[error("Something went wrong.")]
    DomainError(#[from] DomainError),
}
