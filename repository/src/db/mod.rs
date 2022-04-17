pub mod connection;
pub mod params;
pub mod read;
pub mod types;
pub mod write;
use types::*;

pub use connection::Repo;

use once_cell::sync::OnceCell;
pub use params::*;
pub use read::*;
pub use sql_builder::SqlBuilder;
pub use write::*;

static REPOSITORY: OnceCell<Repo> = OnceCell::new();
