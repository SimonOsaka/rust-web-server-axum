pub mod adventures;
pub mod meilisearch;

use meilisearch::MeiliSearch;
use once_cell::sync::OnceCell;

static MEILISEARCH: OnceCell<MeiliSearch> = OnceCell::new();
