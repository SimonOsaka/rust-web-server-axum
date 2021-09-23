use sqlx::Executor;

use crate::db::{PoolOptions, SqlPool};

#[derive(Clone, Debug)]
pub struct Repo {
    pub connection_pool: SqlPool,
}

impl Repo {
    pub async fn new(database_url: &str) -> Self {
        Self::from_pool_builder(&database_url).await
    }

    pub async fn from_pool_builder(database_url: &str) -> Self {
        let connection_pool = PoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .connect_timeout(std::time::Duration::from_secs(30))
            .after_connect(|conn| {
                Box::pin(async move {
                    if cfg!(feature = "mysql") {
                        conn.execute("SET time_zone = '+08:00';").await?;
                    } else if cfg!(feature = "postgres") {
                        conn.execute("SET TIME ZONE '+08:00';").await?;
                    }

                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("init database error");

        debug!("connection pool inited...");
        Repo { connection_pool }
    }
}
