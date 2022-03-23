pub mod connection;
pub mod error;
pub mod operation;

use once_cell::sync::OnceCell;

use self::connection::MeiliSearch;

static MEILISEARCH: OnceCell<MeiliSearch> = OnceCell::new();
