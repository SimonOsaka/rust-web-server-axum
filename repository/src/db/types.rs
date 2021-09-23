// SqlArguments
#[cfg(any(feature = "mysql"))]
pub type SqlArguments = sqlx::mysql::MySqlArguments;

#[cfg(any(feature = "postgres"))]
pub type SqlArguments = sqlx::postgres::PgArguments;

// SqlPool
#[cfg(any(feature = "mysql"))]
pub type SqlPool = sqlx::MySqlPool;
#[cfg(any(feature = "postgres"))]
pub type SqlPool = sqlx::PgPool;

// SqlRow
#[cfg(any(feature = "mysql"))]
pub type SqlRow = sqlx::mysql::MySqlRow;
#[cfg(any(feature = "postgres"))]
pub type SqlRow = sqlx::postgres::PgRow;

// PoolOptions
#[cfg(any(feature = "mysql"))]
pub type PoolOptions = sqlx::mysql::MySqlPoolOptions;
#[cfg(any(feature = "postgres"))]
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
