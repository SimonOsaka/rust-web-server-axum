use extra::meilisearch::error::MeiliSearchError;
use util::i18n::i18n;

#[derive(thiserror::Error, Debug)]
pub enum SearchError {
    #[error("{}({0})", i18n("search-error"))]
    Error(MeiliSearchError),
}

impl From<MeiliSearchError> for SearchError {
    fn from(e: MeiliSearchError) -> Self {
        Self::Error(e)
    }
}
