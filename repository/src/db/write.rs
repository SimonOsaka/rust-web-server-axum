use super::{SqlParams, REPOSITORY};
use sql_builder::SqlBuilder;
use sqlx::{Error, Postgres, Row, Transaction};
use tracing::debug;
use types::ID;

#[async_trait]
pub trait SqlWriter {
    async fn insert_one<'a>(
        &self,
        args: SqlParams,
        transaction: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<ID, Error>;

    async fn update_one<'a>(
        &self,
        args: SqlParams,
        transaction: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<u64, Error>;

    async fn delete_one<'a>(
        &self,
        args: SqlParams,
        transaction: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<u64, Error>;
}

#[async_trait]
impl SqlWriter for SqlBuilder {
    async fn insert_one<'a>(
        &self,
        args: SqlParams,
        transaction: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<ID, Error> {
        let sql = &self.sql().unwrap();
        debug!("insert_one sql: {}", sql);

        let result;
        if let Some(tx) = transaction {
            result = sqlx::query_with(&sql, args.fetch()).fetch_one(tx).await?;
        } else {
            let pool = &REPOSITORY.get().unwrap().connection_pool;
            result = sqlx::query_with(&sql, args.fetch()).fetch_one(pool).await?;
        }
        Ok(result.get(0))
    }

    async fn update_one<'a>(
        &self,
        args: SqlParams,
        transaction: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<u64, Error> {
        let sql = &self.sql().unwrap();
        debug!("update_one sql: {}", sql);

        let result;
        if let Some(tx) = transaction {
            result = sqlx::query_with(&sql, args.fetch()).execute(tx).await?;
        } else {
            let pool = &REPOSITORY.get().unwrap().connection_pool;
            result = sqlx::query_with(&sql, args.fetch()).execute(pool).await?;
        }
        Ok(result.rows_affected())
    }

    async fn delete_one<'a>(
        &self,
        args: SqlParams,
        transaction: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<u64, Error> {
        let sql = &self.sql().unwrap();
        debug!("delete_one sql: {}", sql);

        let result;
        if let Some(tx) = transaction {
            result = sqlx::query_with(&sql, args.fetch()).execute(tx).await?;
        } else {
            let pool = &REPOSITORY.get().unwrap().connection_pool;
            result = sqlx::query_with(&sql, args.fetch()).execute(pool).await?;
        }
        Ok(result.rows_affected())
    }
}
