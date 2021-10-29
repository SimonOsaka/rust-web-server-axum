use sqlx::{Error, Row};
use types::ID;

use super::{types::SqlArguments, REPOSITORY};

pub async fn insert_one(sql: &str, args: SqlArguments) -> Result<ID, Error> {
    let pool = &REPOSITORY.get().unwrap().connection_pool;
    let mut tx = pool.begin().await?;
    let result = sqlx::query_with(&sql, args).fetch_one(&mut tx).await?;
    tx.commit().await?;
    Ok(result.get(0))
}

pub async fn update_one(sql: &str, args: SqlArguments) -> Result<u64, Error> {
    let pool = &REPOSITORY.get().unwrap().connection_pool;
    let mut tx = pool.begin().await?;
    let result = sqlx::query_with(&sql, args).execute(&mut tx).await?;
    tx.commit().await?;
    Ok(result.rows_affected())
}
