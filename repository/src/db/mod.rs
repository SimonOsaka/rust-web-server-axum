pub mod connection;
pub mod query;
pub mod sql_params;
mod types;

use self::types::*;

pub use connection::Repo;

use once_cell::sync::OnceCell;
pub use query::*;
pub use sql_builder::SqlBuilder;
pub use sql_params::SqlParam;

static REPOSITORY: OnceCell<Repo> = OnceCell::new();
