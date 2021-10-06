pub mod connection;
pub mod operation;

use connection::MeiliSearch;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

static MEILISEARCH: OnceCell<MeiliSearch> = OnceCell::new();
