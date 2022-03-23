use meilisearch_sdk::errors::Error;
use util::i18n::i18n;

#[derive(thiserror::Error, Debug)]
pub enum MeiliSearchError {
    #[error("{}({0})", i18n("meilisearch-error"))]
    MeiliSearch(Error),
}

impl From<Error> for MeiliSearchError {
    fn from(e: Error) -> Self {
        Self::MeiliSearch(e)
    }
}
