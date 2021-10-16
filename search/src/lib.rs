pub mod adventures;
pub mod meilisearch;

use meilisearch::MeiliSearch;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

static MEILISEARCH: OnceCell<MeiliSearch> = OnceCell::new();
