pub mod connection;
pub mod params;
pub mod read;
mod types;
pub mod write;
use self::types::*;

pub use connection::Repo;

use once_cell::sync::OnceCell;
pub use params::*;
pub use read::*;
pub use sql_builder::SqlBuilder;
pub use write::*;

static REPOSITORY: OnceCell<Repo> = OnceCell::new();
