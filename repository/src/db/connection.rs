use std::env;

use sqlx::Executor;

use crate::db::{PoolOptions, SqlPool};

use super::REPOSITORY;

#[derive(Clone, Debug)]
pub struct Repo {
    pub(crate) connection_pool: SqlPool,
}

impl Repo {
    async fn new(database_url: &str) -> Self {
        Self::from_pool_builder(&database_url).await
    }

    async fn from_pool_builder(database_url: &str) -> Self {
        let connection_pool = PoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .connect_timeout(std::time::Duration::from_secs(30))
            .after_connect(|conn| {
                Box::pin(async move {
                    conn.execute("SET TIME ZONE 'Asia/Shanghai';").await?;

                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("init database error");

        debug!("connection pool inited...");
        debug!("database_url: {}", database_url);
        Repo { connection_pool }
    }

    pub async fn create() {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let repo = Repo::new(&database_url);

        REPOSITORY.set(repo.await).expect("db connection must set");

        debug!("db connection created");
    }
}
