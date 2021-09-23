use sqlx::{Error, FromRow};

use crate::db::REPO;
use crate::db::{SqlArguments, SqlRow};

pub async fn query_list<T>(sql: &str, args: SqlArguments) -> Result<Vec<T>, Error>
where
    T: for<'r> FromRow<'r, SqlRow> + std::marker::Send + std::marker::Unpin,
{
    let pool = &REPO.get().await.connection_pool;
    let result: Vec<T> = sqlx::query_as_with(&sql, args).fetch_all(pool).await?;

    Ok(result)
}

pub async fn query_one<T>(sql: &str, args: SqlArguments) -> Result<Option<T>, Error>
where
    T: for<'r> sqlx::FromRow<'r, SqlRow> + std::marker::Send + std::marker::Unpin,
{
    let pool = &REPO.get().await.connection_pool;
    let result: Option<T> = sqlx::query_as_with(&sql, args).fetch_optional(pool).await?;

    Ok(result)
}
