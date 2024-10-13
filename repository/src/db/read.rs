use std::future::Future;
use std::marker::Send;
use std::marker::Unpin;

use crate::db::SqlRow;
use crate::db::REPOSITORY;
use sql_builder::SqlBuilder;
use sqlx::{Error, FromRow};
use sqlx::{Postgres, Transaction};
use tracing::debug;

use super::SqlParams;

pub trait SqlReader {
    fn query_list<'c, T>(
        &self,
        args: SqlParams,
        transaction: Option<&mut Transaction<'c, Postgres>>,
    ) -> impl Future<Output = Result<Vec<T>, Error>>
    where
        T: for<'r> FromRow<'r, SqlRow> + Send + Unpin;

    fn query_one_optinal<'c, T>(
        &self,
        args: SqlParams,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> impl Future<Output = Result<Option<T>, Error>>
    where
        T: for<'r> sqlx::FromRow<'r, SqlRow> + Send + Unpin;
}

impl SqlReader for SqlBuilder {
    async fn query_list<'c, T>(
        &self,
        args: SqlParams,
        transaction: Option<&mut Transaction<'c, Postgres>>,
    ) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, SqlRow> + Send + Unpin,
    {
        let sql = &self.sql().unwrap();
        debug!("query_list sql: {}", sql);

        let result: Vec<T> = if let Some(tx) = transaction {
            sqlx::query_as_with(sql, args.fetch())
                .fetch_all(&mut **tx)
                .await?
        } else {
            let pool = &REPOSITORY.get().unwrap().connection_pool;
            sqlx::query_as_with(sql, args.fetch())
                .fetch_all(pool)
                .await?
        };

        Ok(result)
    }

    async fn query_one_optinal<'c, T>(
        &self,
        args: SqlParams,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<Option<T>, Error>
    where
        T: for<'r> sqlx::FromRow<'r, SqlRow> + Send + Unpin,
    {
        let sql = &self.sql().unwrap();
        debug!("query_one sql: {}", sql);

        let result: Option<T> = if let Some(tx) = transaction {
            sqlx::query_as_with(sql, args.fetch())
                .fetch_optional(&mut **tx)
                .await?
        } else {
            let pool = &REPOSITORY.get().unwrap().connection_pool;
            sqlx::query_as_with(sql, args.fetch())
                .fetch_optional(pool)
                .await?
        };

        Ok(result)
    }
}
