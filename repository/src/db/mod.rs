pub mod connection;
pub mod query;
pub mod sql_params;
mod types;

use self::types::*;
use async_once::AsyncOnce;
pub use connection::Repo;
use lazy_static::lazy_static;
pub use query::*;
pub use sql_builder::SqlBuilder;
pub use sql_params::SqlParam;
use std::env;

lazy_static! {
    pub static ref REPO: AsyncOnce<Repo> = AsyncOnce::new(async {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let repo = Repo::new(&database_url);

        repo.await
    });
}
