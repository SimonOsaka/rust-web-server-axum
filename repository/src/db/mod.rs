pub mod connection;
pub mod read;
pub mod sql_params;
mod types;
pub mod write;

use self::types::*;

pub use connection::Repo;

use once_cell::sync::OnceCell;
pub use read::*;
pub use sql_builder::SqlBuilder;
pub use sql_params::SqlParam;
pub use write::*;

static REPOSITORY: OnceCell<Repo> = OnceCell::new();
