use sql_builder::SqlBuilder;
use sqlx::{Error, FromRow};

use crate::db::SqlRow;
use crate::db::REPOSITORY;

use super::SqlParams;

#[async_trait]
pub trait SqlReader {
    async fn query_list<T>(&self, args: SqlParams) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, SqlRow> + std::marker::Send + std::marker::Unpin;

    async fn query_one_optinal<T>(&self, args: SqlParams) -> Result<Option<T>, Error>
    where
        T: for<'r> sqlx::FromRow<'r, SqlRow> + std::marker::Send + std::marker::Unpin;
}

#[async_trait]
impl SqlReader for SqlBuilder {
    async fn query_list<T>(&self, args: SqlParams) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, SqlRow> + std::marker::Send + std::marker::Unpin,
    {
        let sql = &self.sql().unwrap();
        debug!("query_list sql: {}", sql);

        let pool = &REPOSITORY.get().unwrap().connection_pool;
        let result: Vec<T> = sqlx::query_as_with(&sql, args.fetch())
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    async fn query_one_optinal<T>(&self, args: SqlParams) -> Result<Option<T>, Error>
    where
        T: for<'r> sqlx::FromRow<'r, SqlRow> + std::marker::Send + std::marker::Unpin,
    {
        let sql = &self.sql().unwrap();
        debug!("query_one sql: {}", sql);

        let pool = &REPOSITORY.get().unwrap().connection_pool;
        let result: Option<T> = sqlx::query_as_with(&sql, args.fetch())
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }
}
