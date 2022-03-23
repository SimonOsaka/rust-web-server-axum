use anyhow::Error as OpaqueError;
use repository::SqlxError;
use search::adventures::error::SearchError;
use tracing::debug;
use util::i18n::i18n;

pub fn search_to_domain_error(e: SearchError) -> DomainError {
    debug!("search_to_domain_error: {}", e);
    DomainError::from(OpaqueError::from(e))
}

pub fn database_to_domain_error(e: SqlxError) -> DomainError {
    debug!("database_to_domain_error: {}", e);
    DomainError::from(OpaqueError::from(e))
}

#[derive(thiserror::Error, Debug)]
#[error("{}", i18n("something-wrong"))]
pub struct DomainError {
    #[from]
    source: anyhow::Error,
}
