pub mod connection;
pub mod error;
pub mod operation;

pub extern crate meilisearch_sdk;

use once_cell::sync::OnceCell;

use self::connection::MeiliSearch;

static MEILISEARCH: OnceCell<MeiliSearch> = OnceCell::new();
