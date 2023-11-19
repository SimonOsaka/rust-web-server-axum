// use std::fmt::Display;

// SqlArguments
pub type SqlArguments = sqlx::postgres::PgArguments;

// SqlPool
pub type SqlPool = sqlx::PgPool;

// SqlRow
pub type SqlRow = sqlx::postgres::PgRow;

// PoolOptions
pub type PoolOptions = sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub enum Value {
    String(String),
    Integer(i64),
}

// impl Display for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}", self))
//     }
// }

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::Integer(v.into())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(String::from(v))
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

#[derive(Debug)]
pub enum Operation {
    Eq(Value),
    Between(Value, Value),
}
