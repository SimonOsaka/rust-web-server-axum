use sql_builder::SqlBuilder;
use sqlx::{Error, Row};
use types::ID;

use super::{SqlParams, REPOSITORY};

#[async_trait]
pub trait SqlWriter {
    async fn insert_one(&self, args: SqlParams) -> Result<ID, Error>;
    async fn update_one(&self, args: SqlParams) -> Result<u64, Error>;
    async fn delete_one(&self, args: SqlParams) -> Result<u64, Error>;
}

#[async_trait]
impl SqlWriter for SqlBuilder {
    async fn insert_one(&self, args: SqlParams) -> Result<ID, Error> {
        let sql = &self.sql().unwrap();
        debug!("insert_one sql: {}", sql);

        let pool = &REPOSITORY.get().unwrap().connection_pool;
        let mut tx = pool.begin().await?;
        let result = sqlx::query_with(&sql, args.fetch())
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(result.get(0))
    }

    async fn update_one(&self, args: SqlParams) -> Result<u64, Error> {
        let sql = &self.sql().unwrap();
        debug!("update_one sql: {}", sql);

        let pool = &REPOSITORY.get().unwrap().connection_pool;
        let mut tx = pool.begin().await?;
        let result = sqlx::query_with(&sql, args.fetch())
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(result.rows_affected())
    }

    async fn delete_one(&self, args: SqlParams) -> Result<u64, Error> {
        let sql = &self.sql().unwrap();
        debug!("delete_one sql: {}", sql);

        let pool = &REPOSITORY.get().unwrap().connection_pool;
        let mut tx = pool.begin().await?;
        let result = sqlx::query_with(&sql, args.fetch())
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(result.rows_affected())
    }
}
