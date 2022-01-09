use i18n::{i18n, i18n_with_vars};
use types::ID;

use crate::DomainError;

#[derive(thiserror::Error, Debug)]
pub enum GetAdventureError {
    #[error("{}", i18n_with_vars("adventure-not-exist", vec![adventure_id.to_string()]))]
    NotFound {
        adventure_id: ID,
        // source: DomainError,
    },
    #[error("{}", i18n("something-wrong"))]
    DomainError(#[from] DomainError),
}

#[derive(thiserror::Error, Debug)]
pub enum CreateAdventureError {
    #[error("{}", i18n_with_vars("adventure-not-exist", vec![adventure_id.to_string()]))]
    AdventureNotFound {
        adventure_id: ID,
        // source: DomainError,
    },
    #[error("{}", i18n("adventure-exist"))]
    Exist,
    #[error("{}", i18n("adventure-add-document-error"))]
    AddDocuments,
    #[error("{}", i18n("something-wrong"))]
    DomainError(#[from] DomainError),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteAdventureError {
    #[error("{}", i18n_with_vars("adventure-not-exist", vec![adventure_id.to_string()]))]
    AdventureNotFound {
        adventure_id: ID,
        // source: DomainError,
    },
    #[error("{}", i18n("adventure-owner-wrong"))]
    NotOwner,
    #[error("{}", i18n("adventure-del-document-error"))]
    DelDocuments,
    #[error("{}", i18n("something-wrong"))]
    DomainError(#[from] DomainError),
}
