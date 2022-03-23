use self::connection::RedisConnection;
use once_cell::sync::OnceCell;

pub mod connection;
pub mod operation;

static REDIS: OnceCell<RedisConnection> = OnceCell::new();
